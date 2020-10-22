use xaynet_core::{
    crypto::{EncryptKeyPair, Signature},
    message::Sum as SumMessage,
};

use super::{Phase, Progress, SharedState, Step, Sum2, TransitionOutcome};
use crate::{state_machine::io::StateMachineIO, utils::multipart::MessageEncoder};

#[derive(Serialize, Deserialize, Debug)]
pub struct Sum {
    pub ephm_keys: EncryptKeyPair,
    pub sum_signature: Signature,
    pub message: Option<MessageEncoder>,
}

#[async_trait]
impl<IO> Step<IO> for Phase<Sum, IO>
where
    IO: StateMachineIO,
{
    async fn step(mut self) -> TransitionOutcome<IO> {
        info!("sum task");

        self = try_progress!(self.compose_sum_message());

        // FIXME: currently if sending fails, we lose the message,
        // thus wasting all the work we've done in this phase
        let message = self.phase_state.message.take().unwrap();
        match self.send_message(message).await {
            Ok(_) => {
                info!("sent sum message, going to sum2 phase");
                TransitionOutcome::Complete(self.into_sum2().into())
            }
            Err(e) => {
                warn!("failed to send sum message: {}", e);
                warn!("sum phase failed, going back to awaiting phase");
                TransitionOutcome::Complete(self.into_awaiting().into())
            }
        }
    }
}

impl<IO> Phase<Sum, IO>
where
    IO: StateMachineIO,
{
    pub fn new(shared_state: SharedState, io: IO, sum_signature: Signature) -> Self {
        Self {
            phase_state: Sum {
                ephm_keys: EncryptKeyPair::generate(),
                sum_signature,
                message: None,
            },
            shared_state,
            io,
        }
    }

    pub fn compose_sum_message(mut self) -> Progress<Sum, IO> {
        if self.phase_state.message.is_some() {
            return Progress::Continue(self);
        }

        let sum = SumMessage {
            sum_signature: self.phase_state.sum_signature,
            ephm_pk: self.phase_state.ephm_keys.public,
        };
        self.phase_state.message = Some(self.message_encoder(sum.into()));
        Progress::Updated(self.into())
    }

    pub fn into_sum2(self) -> Phase<Sum2, IO> {
        Phase::<Sum2, IO>::new(self.shared_state, self.io, self.phase_state)
    }
}
