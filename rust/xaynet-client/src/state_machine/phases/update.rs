use xaynet_core::{
    crypto::Signature,
    mask::{MaskObject, MaskSeed, Masker, Model},
    message::Update as UpdateMessage,
    LocalSeedDict,
    ParticipantTaskSignature,
    SumDict,
};

use super::{Phase, Progress, Step};
use crate::{state_machine::TransitionOutcome, MessageEncoder, IO};

#[derive(Serialize, Deserialize, Debug)]
pub struct Update {
    pub sum_signature: ParticipantTaskSignature,
    pub update_signature: ParticipantTaskSignature,
    pub sum_dict: Option<SumDict>,
    pub seed_dict: Option<LocalSeedDict>,
    pub model: Option<Model>,
    pub mask: Option<(MaskSeed, MaskObject)>,
    pub message: Option<MessageEncoder>,
}

impl Update {
    pub fn new(sum_signature: Signature, update_signature: Signature) -> Self {
        Update {
            sum_signature,
            update_signature,
            sum_dict: None,
            seed_dict: None,
            model: None,
            mask: None,
            message: None,
        }
    }

    fn has_fetched_sum_dict(&self) -> bool {
        self.sum_dict.is_some() || self.has_loaded_model()
    }

    fn has_loaded_model(&self) -> bool {
        self.model.is_some() || self.has_masked_model()
    }

    fn has_masked_model(&self) -> bool {
        self.mask.is_some() || self.has_built_seed_dict()
    }

    fn has_built_seed_dict(&self) -> bool {
        self.seed_dict.is_some() || self.has_composed_message()
    }

    fn has_composed_message(&self) -> bool {
        self.message.is_some()
    }
}

#[async_trait]
impl Step for Phase<Update> {
    async fn step(mut self) -> TransitionOutcome {
        self = try_progress!(self.fetch_sum_dict().await);
        self = try_progress!(self.load_model().await);
        self = try_progress!(self.mask_model());
        self = try_progress!(self.build_seed_dict());
        self = try_progress!(self.compose_update_message());

        // FIXME: currently if sending fails, we lose the message,
        // thus wasting all the work we've done in this phase
        let message = self.state.phase.message.take().unwrap();
        match self.send_message(message).await {
            Ok(_) => {
                info!("sent update message");
            }
            Err(e) => {
                warn!("failed to send update message: {}", e);
                warn!("update phase failed");
            }
        }

        info!("going back to awaiting phase");
        TransitionOutcome::Complete(self.into_awaiting().into())
    }
}

impl Phase<Update> {
    async fn fetch_sum_dict(mut self) -> Progress<Update> {
        if self.state.phase.has_fetched_sum_dict() {
            return Progress::Continue(self);
        }
        debug!("fetching sum dictionary");
        match self.io.get_sums().await {
            Ok(Some(dict)) => {
                self.state.phase.sum_dict = Some(dict);
                Progress::Updated(self.into())
            }
            Ok(None) => {
                debug!("sum dictionary is not available yet");
                Progress::Stuck(self)
            }
            Err(e) => {
                warn!("failed to fetch sum dictionary: {:?}", e);
                Progress::Stuck(self)
            }
        }
    }

    async fn load_model(mut self) -> Progress<Update> {
        if self.state.phase.has_loaded_model() {
            return Progress::Continue(self);
        }

        debug!("loading local model");
        match self.io.load_model().await {
            Ok(Some(model)) => {
                self.state.phase.model = Some(model);
                Progress::Updated(self.into())
            }
            Ok(None) => {
                debug!("model is not ready");
                Progress::Stuck(self)
            }
            Err(e) => {
                warn!("failed to load model: {:?}", e);
                Progress::Stuck(self)
            }
        }
    }

    /// Generate a mask seed and mask a local model.
    fn mask_model(mut self) -> Progress<Update> {
        if self.state.phase.has_masked_model() {
            return Progress::Continue(self);
        }
        let config = self.state.shared.mask_config;
        let masker = Masker::new(config, config);
        let model = self.state.phase.model.take().unwrap();
        let scalar = self.state.shared.scalar;
        self.state.phase.mask = Some(masker.mask(scalar, model));
        Progress::Updated(self.into())
    }

    // Create a local seed dictionary from a sum dictionary.
    fn build_seed_dict(mut self) -> Progress<Update> {
        if self.state.phase.has_built_seed_dict() {
            return Progress::Continue(self);
        }
        let mask_seed = &self.state.phase.mask.as_ref().unwrap().0;
        debug!("building local seed dictionary");
        let seeds = self
            .state
            .phase
            .sum_dict
            .take()
            .unwrap()
            .into_iter()
            .map(|(pk, ephm_pk)| (pk, mask_seed.encrypt(&ephm_pk)))
            .collect();
        self.state.phase.seed_dict = Some(seeds);
        Progress::Updated(self.into())
    }

    fn compose_update_message(mut self) -> Progress<Update> {
        debug!("composing update message");
        let update = UpdateMessage {
            sum_signature: self.state.phase.sum_signature,
            update_signature: self.state.phase.update_signature,
            masked_model: self.state.phase.mask.take().unwrap().1,
            local_seed_dict: self.state.phase.seed_dict.take().unwrap(),
        };
        self.state.phase.message = Some(self.message_encoder(update.into()));
        Progress::Updated(self.into())
    }
}
