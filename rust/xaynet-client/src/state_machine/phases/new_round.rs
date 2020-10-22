use xaynet_core::crypto::{ByteObject, Signature};

use super::{Phase, SharedState, Step, Sum, TransitionOutcome, Update};
use crate::state_machine::io::StateMachineIO;

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
        if sum_signature.is_eligible(self.shared_state.round_params.sum) {
            info!("eligible for sum task");
            return TransitionOutcome::Complete(self.into_sum(sum_signature).into());
        }

        info!("not eligible for sum task, checking eligibility for update task");
        let update_signature = self.sign(b"update");
        if update_signature.is_eligible(self.shared_state.round_params.update) {
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
    pub fn new(shared_state: SharedState, io: IO) -> Self {
        Phase {
            shared_state,
            io,
            phase_state: NewRound,
        }
    }

    fn sign(&self, data: &[u8]) -> Signature {
        let sk = &self.shared_state.keys.secret;
        let seed = self.shared_state.round_params.seed.as_slice();
        sk.sign_detached(&[seed, data].concat())
    }

    fn into_sum(self, sum_signature: Signature) -> Phase<Sum, IO> {
        Phase::<Sum, IO>::new(self.shared_state, self.io, sum_signature)
    }

    fn into_update(
        self,
        sum_signature: Signature,
        update_signature: Signature,
    ) -> Phase<Update, IO> {
        Phase::<Update, IO>::new(self.shared_state, self.io, sum_signature, update_signature)
    }
}
