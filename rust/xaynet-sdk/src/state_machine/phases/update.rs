use std::ops::Deref;

use derive_more::From;
use xaynet_core::{
    crypto::Signature,
    mask::{MaskObject, MaskSeed, Masker, Model},
    message::Update as UpdateMessage,
    LocalSeedDict,
    ParticipantTaskSignature,
    SumDict,
};

use crate::{
    state_machine::{Phase, Progress, Step, TransitionOutcome, IO},
    MessageEncoder,
};

#[derive(From)]
pub enum LocalModel {
    Dyn(Box<dyn AsRef<Model> + Send>),
    Owned(Model),
}

impl AsRef<Model> for LocalModel {
    fn as_ref(&self) -> &Model {
        match self {
            LocalModel::Dyn(model) => model.deref().as_ref(),
            LocalModel::Owned(model) => model,
        }
    }
}

impl serde::ser::Serialize for LocalModel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        match self {
            LocalModel::Dyn(model) => model.as_ref().as_ref().serialize(serializer),
            LocalModel::Owned(model) => model.serialize(serializer),
        }
    }
}

impl<'de> serde::de::Deserialize<'de> for LocalModel {
    fn deserialize<D>(deserializer: D) -> Result<LocalModel, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let model = <Model as serde::de::Deserialize>::deserialize(deserializer)?;
        Ok(LocalModel::Owned(model))
    }
}

#[derive(Serialize, Deserialize)]
pub struct Update {
    pub sum_signature: ParticipantTaskSignature,
    pub update_signature: ParticipantTaskSignature,
    pub sum_dict: Option<SumDict>,
    pub seed_dict: Option<LocalSeedDict>,
    pub model: Option<LocalModel>,
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
        let message = self.state.private.message.take().unwrap();
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
        self.io.notify_idle();
        TransitionOutcome::Complete(self.into_awaiting().into())
    }
}

impl Phase<Update> {
    async fn fetch_sum_dict(mut self) -> Progress<Update> {
        if self.state.private.has_fetched_sum_dict() {
            debug!("already fetched the sum dictionary, continuing");
            return Progress::Continue(self);
        }
        debug!("fetching sum dictionary");
        match self.io.get_sums().await {
            Ok(Some(dict)) => {
                self.state.private.sum_dict = Some(dict);
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
        if self.state.private.has_loaded_model() {
            return Progress::Continue(self);
        }

        debug!("loading local model");
        match self.io.load_model().await {
            Ok(Some(model)) => {
                self.state.private.model = Some(model.into());
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
        if self.state.private.has_masked_model() {
            debug!("already computed the masked model, continuing");
            return Progress::Continue(self);
        }
        info!("computing masked model");
        let config = self.state.shared.mask_config;
        let masker = Masker::new(config, config);
        let model = self.state.private.model.take().unwrap();
        let scalar = self.state.shared.scalar;
        self.state.private.mask = Some(masker.mask(scalar, model.as_ref()));
        Progress::Updated(self.into())
    }

    // Create a local seed dictionary from a sum dictionary.
    fn build_seed_dict(mut self) -> Progress<Update> {
        if self.state.private.has_built_seed_dict() {
            debug!("already built the seed dictionary, continuing");
            return Progress::Continue(self);
        }
        let mask_seed = &self.state.private.mask.as_ref().unwrap().0;
        info!("building local seed dictionary");
        let seeds = self
            .state
            .private
            .sum_dict
            .take()
            .unwrap()
            .into_iter()
            .map(|(pk, ephm_pk)| (pk, mask_seed.encrypt(&ephm_pk)))
            .collect();
        self.state.private.seed_dict = Some(seeds);
        Progress::Updated(self.into())
    }

    fn compose_update_message(mut self) -> Progress<Update> {
        if self.state.private.has_composed_message() {
            debug!("already composed the update message, continuing");
            return Progress::Continue(self);
        }
        debug!("composing update message");
        let update = UpdateMessage {
            sum_signature: self.state.private.sum_signature,
            update_signature: self.state.private.update_signature,
            masked_model: self.state.private.mask.take().unwrap().1,
            local_seed_dict: self.state.private.seed_dict.take().unwrap(),
        };
        self.state.private.message = Some(self.message_encoder(update.into()));
        Progress::Updated(self.into())
    }
}