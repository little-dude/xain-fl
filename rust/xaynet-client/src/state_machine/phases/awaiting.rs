use super::{Phase, Step};
use crate::state_machine::TransitionOutcome;

#[derive(Serialize, Deserialize, Debug)]
pub struct Awaiting;

#[async_trait]
impl Step for Phase<Awaiting> {
    async fn step(mut self) -> TransitionOutcome {
        info!("awaiting task");
        return TransitionOutcome::Pending(self.into());
    }
}
