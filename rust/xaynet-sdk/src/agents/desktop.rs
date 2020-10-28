use std::time::Duration;

use crate::{
    settings::Settings,
    state_machine::{StateMachine, TransitionOutcome},
    ModelStore,
    XaynetClient,
    Notify,
};

use tokio::time::delay_for;

pub struct Agent(StateMachine);

impl Agent {
    pub fn new<X, M, N>(settings: Settings, xaynet_client: X, model_store: M, notify: N) -> Self
    where
        X: XaynetClient + Send + 'static,
        M: ModelStore + Send + 'static,
        N: Notify + Send + 'static,
    {
        Agent(StateMachine::new(settings, xaynet_client, model_store, notify))
    }

    pub async fn run(mut self, tick: Duration) {
        loop {
            self = match self.0.transition().await {
                TransitionOutcome::Pending(state_machine) => {
                    delay_for(tick.clone()).await;
                    Self(state_machine)
                }
                TransitionOutcome::Complete(state_machine) => Self(state_machine),
            };
        }
    }
}
