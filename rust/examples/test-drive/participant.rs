use std::sync::Arc;
use tokio::sync::mpsc;

use xaynet_client::Client;
use xaynet_core::mask::Model;
use xaynet_sdk::{agents::desktop::Agent, AgentSettings, ModelStore, Notify, XaynetClient};

enum AgentNotification {
    Update,
    Sum,
    NewRound,
    Idle,
}

pub struct Participant {
    // FIXME: XaynetClient requires the client to be mutable. This may
    // make it easier to implement clients, but as a result we can't
    // wrap the client in an Arc, which would allow us to share the
    // same client with all the participants. Maybe XaynetClient
    // should have methods that take &self?
    xaynet_client: Client,
    notifications: mpsc::Receiver<AgentNotification>,
}

impl Participant {
    pub fn new(settings: AgentSettings, xaynet_client: Client, model: Arc<Model>) -> (Self, Agent) {
        let (tx, rx) = mpsc::channel::<AgentNotification>(10);
        let notifier = Notifier(tx);
        let agent = Agent::new(settings, xaynet_client.clone(), LocalModel(model), notifier);
        let participant = Self {
            xaynet_client,
            notifications: rx,
        };
        (participant, agent)
    }

    pub async fn run(mut self) {
        use AgentNotification::*;
        loop {
            match self.notifications.recv().await {
                Some(Sum) => {
                    info!("taking part to the sum task");
                }
                Some(Update) => {
                    info!("taking part to the update task");
                }
                Some(Idle) => {
                    info!("waiting");
                }
                Some(NewRound) => {
                    info!("new round started, downloading latest global weights");
                    if let Err(e) = self.xaynet_client.get_model().await {
                        warn!("failed to download latest model: {}", e);
                    }
                }
                None => {
                    warn!("notifications channel closed, terminating");
                    return;
                }
            }
        }
    }
}

struct Notifier(mpsc::Sender<AgentNotification>);

impl Notify for Notifier {
    fn notify_new_round(&mut self) {
        if let Err(e) = self.0.try_send(AgentNotification::NewRound) {
            warn!("failed to notify participant: {}", e);
        }
    }
    fn notify_sum(&mut self) {
        if let Err(e) = self.0.try_send(AgentNotification::Sum) {
            warn!("failed to notify participant: {}", e);
        }
    }
    fn notify_update(&mut self) {
        if let Err(e) = self.0.try_send(AgentNotification::Update) {
            warn!("failed to notify participant: {}", e);
        }
    }
    fn notify_idle(&mut self) {
        if let Err(e) = self.0.try_send(AgentNotification::Idle) {
            warn!("failed to notify participant: {}", e);
        }
    }
}

pub struct LocalModel(Arc<Model>);

#[async_trait]
impl ModelStore for LocalModel {
    type Model = Arc<Model>;
    type Error = std::convert::Infallible;

    async fn load_model(&mut self) -> Result<Option<Self::Model>, Self::Error> {
        Ok(Some(self.0.clone()))
    }
}
