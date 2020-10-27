use derive_more::From;
use thiserror::Error;

use xaynet_core::{common::RoundParameters, crypto::SigningKeyPair, message::Payload};

use super::{Awaiting, NewRound, Sum, Sum2, Update};
use crate::{
    settings::Settings,
    state_machine::{io::StateMachineIO, StateMachine, TransitionOutcome},
    MessageEncoder,
};

#[derive(Debug)]
pub struct Phase<S, IO> {
    /// State that is specific to this phase
    pub phase_state: S,
    /// State shared by all the phases
    pub shared_state: SharedState,
    /// Opaque client for performing IO tasks: talking with the
    /// coordinator API, loading models, etc.
    pub io: IO,
}

/// Store for all the data that are common to all the phases
#[derive(Serialize, Deserialize, Debug)]
pub struct SharedState {
    pub settings: Settings,
    pub keys: SigningKeyPair,
    pub round_params: RoundParameters,
}

/// Represent an atomic step performed by a phase. If the step results
/// in progress being made, the resulting updated state machine is
/// returned as `Progress::Some`.
#[async_trait]
pub trait Step<IO> {
    async fn step(mut self) -> TransitionOutcome<IO>;
}

#[macro_export]
macro_rules! try_progress {
    ($progress:expr) => {{
        use $crate::state_machine::{phases::Progress, TransitionOutcome};
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
pub enum Progress<S, IO> {
    /// No progress can be made currently.
    Stuck(Phase<S, IO>),
    /// More work needs to be done for progress to be made
    Continue(Phase<S, IO>),
    /// Progress has been made and resulted in this new state machine
    Updated(StateMachine<IO>),
}

impl<S, IO> Phase<S, IO>
where
    IO: StateMachineIO,
    Phase<S, IO>: Step<IO> + Into<StateMachine<IO>>,
{
    pub async fn step(mut self) -> TransitionOutcome<IO> {
        match self.check_round_freshness().await {
            RoundFreshness::Unknown => TransitionOutcome::Pending(self.into()),
            RoundFreshness::Outdated => {
                info!("a new round started: updating the round parameters and resetting the state machine");
                TransitionOutcome::Complete(
                    Phase::<NewRound, IO>::new(self.shared_state, self.io).into(),
                )
            }
            RoundFreshness::Fresh => {
                debug!("round is still fresh, continuing from where we left off");
                <Self as Step<_>>::step(self).await
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
                if params == self.shared_state.round_params {
                    debug!("round parameters didn't change");
                    RoundFreshness::Fresh
                } else {
                    info!("fetched fresh round parameters");
                    self.shared_state.round_params = params;
                    RoundFreshness::Outdated
                }
            }
        }
    }
}

impl<S, IO> Phase<S, IO> {
    pub fn restore(shared_state: SharedState, phase_state: S, io: IO) -> Self {
        Self {
            shared_state,
            phase_state,
            io,
        }
    }
}

impl<S, IO> Phase<S, IO>
where
    IO: StateMachineIO,
{
    pub(super) fn into_awaiting(self) -> Phase<Awaiting, IO> {
        Phase::<Awaiting, IO>::new(self.shared_state, self.io)
    }

    pub(super) async fn send_message(
        &mut self,
        encoder: MessageEncoder,
    ) -> Result<(), SendMessageError> {
        for part in encoder {
            let data = self.shared_state.round_params.pk.encrypt(part.as_slice());
            self.io.send_message(data).await.map_err(|e| {
                error!("failed to send message: {:?}", e);
                SendMessageError
            })?
        }
        Ok(())
    }

    pub(super) fn message_encoder(&self, payload: Payload) -> MessageEncoder {
        MessageEncoder::new(
            self.shared_state.keys.clone(),
            payload,
            self.shared_state.round_params.pk,
            self.shared_state
                .settings
                .max_message_size
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

#[derive(Serialize, Deserialize)]
pub struct SerializableState {
    pub shared: SharedState,
    pub phase: PhaseState,
}

#[derive(Serialize, Deserialize, From)]
pub enum PhaseState {
    NewRound(NewRound),
    Awaiting(Awaiting),
    Sum(Sum),
    Update(Update),
    Sum2(Sum2),
}

impl<IO, T> Into<SerializableState> for Phase<T, IO>
where
    T: Into<PhaseState>,
{
    fn into(self) -> SerializableState {
        SerializableState {
            shared: self.shared_state,
            phase: self.phase_state.into(),
        }
    }
}
