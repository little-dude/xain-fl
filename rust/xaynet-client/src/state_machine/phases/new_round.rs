use xaynet_core::crypto::{ByteObject, Signature};

use super::{Phase, State, Step, Sum, Update};
use crate::state_machine::{io::StateMachineIO, TransitionOutcome};

#[derive(Serialize, Deserialize, Debug)]
pub struct NewRound;

#[async_trait]
impl<IO> Step<IO> for Phase<NewRound, IO>
where
    IO: StateMachineIO,
{
    async fn step(mut self) -> TransitionOutcome<IO> {
        info!("new_round task");

        info!("checking eligibility for sum task");
        let sum_signature = self.sign(b"sum");
        if sum_signature.is_eligible(self.state.shared.round_params.sum) {
            info!("eligible for sum task");
            return TransitionOutcome::Complete(self.into_sum(sum_signature).into());
        }

        info!("not eligible for sum task, checking eligibility for update task");
        let update_signature = self.sign(b"update");
        if update_signature.is_eligible(self.state.shared.round_params.update) {
            info!("eligible for update task");
            return TransitionOutcome::Complete(
                self.into_update(sum_signature, update_signature).into(),
            );
        }

        info!("not eligible for update task, going to sleep until next round");
        return TransitionOutcome::Complete(self.into_awaiting().into());
    }
}

impl<IO> Phase<NewRound, IO>
where
    IO: StateMachineIO,
{
    fn sign(&self, data: &[u8]) -> Signature {
        let sk = &self.state.shared.keys.secret;
        let seed = self.state.shared.round_params.seed.as_slice();
        sk.sign_detached(&[seed, data].concat())
    }

    fn into_sum(self, sum_signature: Signature) -> Phase<Sum, IO> {
        let sum = Sum::new(sum_signature);
        Phase::<Sum, IO>::new(State::new(self.state.shared, sum), self.io)
    }

    fn into_update(
        self,
        sum_signature: Signature,
        update_signature: Signature,
    ) -> Phase<Update, IO> {
        let update = Update::new(sum_signature, update_signature);
        Phase::<Update, IO>::new(State::new(self.state.shared, update), self.io)
    }
}
