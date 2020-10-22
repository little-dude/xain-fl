use super::{Phase, SharedState, Step, TransitionOutcome};
use crate::state_machine::StateMachineIO;

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

impl<IO> Phase<Awaiting, IO> {
    pub fn new(shared_state: SharedState, io: IO) -> Self {
        Phase {
            shared_state,
            io,
            phase_state: Awaiting,
        }
    }
}
