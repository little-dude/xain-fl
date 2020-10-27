use super::{Phase, Step};
use crate::state_machine::{StateMachineIO, TransitionOutcome};

#[derive(Serialize, Deserialize, Debug)]
pub struct Awaiting;

#[async_trait]
impl<IO> Step<IO> for Phase<Awaiting, IO>
where
    IO: StateMachineIO,
{
    async fn step(mut self) -> TransitionOutcome<IO> {
        info!("awaiting task");
        return TransitionOutcome::Pending(self.into());
    }
}
