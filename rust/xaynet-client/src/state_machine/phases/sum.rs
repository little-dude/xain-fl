use xaynet_core::{
    crypto::{EncryptKeyPair, Signature},
    message::Sum as SumMessage,
};

use super::{Phase, Progress, State, Step, Sum2};
use crate::{
    state_machine::{io::StateMachineIO, TransitionOutcome},
    MessageEncoder,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Sum {
    pub ephm_keys: EncryptKeyPair,
    pub sum_signature: Signature,
    pub message: Option<MessageEncoder>,
}

impl Sum {
    pub fn new(sum_signature: Signature) -> Self {
        Sum {
            ephm_keys: EncryptKeyPair::generate(),
            sum_signature,
            message: None,
        }
    }
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
        let message = self.state.phase.message.take().unwrap();
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
    pub fn compose_sum_message(mut self) -> Progress<Sum, IO> {
        if self.state.phase.message.is_some() {
            return Progress::Continue(self);
        }

        let sum = SumMessage {
            sum_signature: self.state.phase.sum_signature,
            ephm_pk: self.state.phase.ephm_keys.public,
        };
        self.state.phase.message = Some(self.message_encoder(sum.into()));
        Progress::Updated(self.into())
    }

    pub fn into_sum2(self) -> Phase<Sum2, IO> {
        let sum2 = Sum2::new(self.state.phase.ephm_keys, self.state.phase.sum_signature);
        Phase::<Sum2, IO>::new(State::new(self.state.shared, sum2), self.io)
    }
}
