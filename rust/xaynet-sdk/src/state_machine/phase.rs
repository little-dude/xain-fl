use derive_more::From;
use thiserror::Error;

use xaynet_core::{
    common::RoundParameters,
    crypto::SigningKeyPair,
    mask::{MaskConfig, Model},
    message::Payload,
};

use super::{Awaiting, NewRound, Sum, Sum2, Update, IO};
use crate::{
    settings::{AgentSettings, MaxMessageSize},
    state_machine::{StateMachine, TransitionOutcome},
    MessageEncoder,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct State<P> {
    /// data specific to the current phase
    pub private: P,
    /// data common to most of the phases
    pub shared: SharedState,
}

impl<P> State<P> {
    pub fn new(shared: SharedState, private: P) -> Self {
        Self { shared, private }
    }
}

pub struct Phase<P> {
    pub(in crate::state_machine) state: State<P>,
    /// Opaque client for performing IO tasks: talking with the
    /// coordinator API, loading models, etc.
    pub(in crate::state_machine) io: Box<dyn IO<Model = Box<dyn AsRef<Model> + Send>>>,
}

/// Store for all the data that are common to all the phases
#[derive(Serialize, Deserialize, Debug)]
pub struct SharedState {
    pub keys: SigningKeyPair,
    pub mask_config: MaskConfig,
    pub scalar: f64,
    pub message_size: MaxMessageSize,
    pub round_params: RoundParameters,
}

impl SharedState {
    pub fn new(settings: AgentSettings) -> Self {
        Self {
            keys: settings.keys,
            mask_config: settings.mask_config,
            scalar: settings.scalar,
            message_size: settings.max_message_size,
            round_params: RoundParameters::default(),
        }
    }
}

/// Represent an atomic step performed by a phase. If the step results
/// in progress being made, the resulting updated state machine is
/// returned as `Progress::Some`.
#[async_trait]
pub trait Step {
    async fn step(mut self) -> TransitionOutcome;
}

#[macro_export]
macro_rules! try_progress {
    ($progress:expr) => {{
        use $crate::state_machine::{Progress, TransitionOutcome};
        match $progress {
            // No progress can be made. Return the state machine as is
            Progress::Stuck(phase) => return TransitionOutcome::Pending(phase.into()),
            // Further progress can be made but require more work, so don't return
            Progress::Continue(phase) => phase,
            // Progress has been made, return the updated state machine
            Progress::Updated(state_machine) => return TransitionOutcome::Complete(state_machine),
        }
    }};
}

/// Represent the presence of absence of progress being made during a phase.
pub enum Progress<P> {
    /// No progress can be made currently.
    Stuck(Phase<P>),
    /// More work needs to be done for progress to be made
    Continue(Phase<P>),
    /// Progress has been made and resulted in this new state machine
    Updated(StateMachine),
}

impl<P> Phase<P>
where
    Phase<P>: Step + Into<StateMachine>,
{
    pub async fn step(mut self) -> TransitionOutcome {
        match self.check_round_freshness().await {
            RoundFreshness::Unknown => TransitionOutcome::Pending(self.into()),
            RoundFreshness::Outdated => {
                info!("a new round started: updating the round parameters and resetting the state machine");
                self.io.notify_new_round();
                TransitionOutcome::Complete(
                    Phase::<NewRound>::new(State::new(self.state.shared, NewRound), self.io).into(),
                )
            }
            RoundFreshness::Fresh => {
                debug!("round is still fresh, continuing from where we left off");
                <Self as Step>::step(self).await
            }
        }
    }

    async fn check_round_freshness(&mut self) -> RoundFreshness {
        match self.io.get_round_params().await {
            Err(e) => {
                warn!("failed to fetch round parameters {:?}", e);
                RoundFreshness::Unknown
            }
            Ok(params) => {
                if params == self.state.shared.round_params {
                    debug!("round parameters didn't change");
                    RoundFreshness::Fresh
                } else {
                    info!("fetched fresh round parameters");
                    self.state.shared.round_params = params;
                    RoundFreshness::Outdated
                }
            }
        }
    }
}

impl<P> Phase<P> {
    pub(in crate::state_machine) fn new(
        state: State<P>,
        io: Box<dyn IO<Model = Box<dyn AsRef<Model> + Send>>>,
    ) -> Self {
        Self { state, io }
    }
}

impl<P> Phase<P> {
    pub(super) fn into_awaiting(self) -> Phase<Awaiting> {
        Phase::<Awaiting>::new(State::new(self.state.shared, Awaiting), self.io)
    }

    pub(super) async fn send_message(
        &mut self,
        encoder: MessageEncoder,
    ) -> Result<(), SendMessageError> {
        for part in encoder {
            let data = self.state.shared.round_params.pk.encrypt(part.as_slice());
            self.io.send_message(data).await.map_err(|e| {
                error!("failed to send message: {:?}", e);
                SendMessageError
            })?
        }
        Ok(())
    }

    pub(super) fn message_encoder(&self, payload: Payload) -> MessageEncoder {
        MessageEncoder::new(
            self.state.shared.keys.clone(),
            payload,
            self.state.shared.round_params.pk,
            self.state
                .shared
                .message_size
                .max_payload_size()
                .unwrap_or(0),
        )
        // the encoder rejects Chunk payload, but in the state
        // machine, we never manually create such payloads so
        // unwrapping is fine
        .unwrap()
    }
}

#[derive(Error, Debug)]
#[error("failed to send a PET message")]
pub struct SendMessageError;

pub enum RoundFreshness {
    Outdated,
    Unknown,
    Fresh,
}

/// A serializable representation of a phase state.
#[derive(Serialize, Deserialize, From)]
pub enum SerializableState {
    NewRound(State<NewRound>),
    Awaiting(State<Awaiting>),
    Sum(State<Sum>),
    // FIXME: this should be boxed...
    #[allow(clippy::large_enum_variant)]
    Update(State<Update>),
    Sum2(State<Sum2>),
}

impl<P> Into<SerializableState> for Phase<P>
where
    State<P>: Into<SerializableState>,
{
    fn into(self) -> SerializableState {
        self.state.into()
    }
}
