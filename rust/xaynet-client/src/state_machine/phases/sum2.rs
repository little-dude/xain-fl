use xaynet_core::{
    crypto::{EncryptKeyPair, Signature},
    mask::{Aggregation, MaskObject, MaskSeed},
    message::Sum2 as Sum2Message,
    UpdateSeedDict,
};

use super::{Phase, Progress, SharedState, Step, Sum, TransitionOutcome};
use crate::{state_machine::io::StateMachineIO, utils::multipart::MessageEncoder};

#[derive(Serialize, Deserialize, Debug)]
pub struct Sum2 {
    ephm_keys: EncryptKeyPair,
    sum_signature: Signature,
    seed_dict: Option<UpdateSeedDict>,
    seeds: Option<Vec<MaskSeed>>,
    mask: Option<MaskObject>,
    mask_length: Option<u64>,
    message: Option<MessageEncoder>,
}

impl From<Sum> for Sum2 {
    fn from(sum: Sum) -> Self {
        Self {
            ephm_keys: sum.ephm_keys,
            sum_signature: sum.sum_signature,
            seed_dict: None,
            seeds: None,
            mask: None,
            mask_length: None,
            message: None,
        }
    }
}

impl Sum2 {
    fn has_fetched_seed_dict(&self) -> bool {
        self.seed_dict.is_some() || self.has_fetched_mask_length()
    }

    fn has_fetched_mask_length(&self) -> bool {
        self.mask_length.is_some() || self.has_decrypted_seeds()
    }

    fn has_decrypted_seeds(&self) -> bool {
        self.seeds.is_some() || self.has_aggregated_masks()
    }

    fn has_aggregated_masks(&self) -> bool {
        self.mask.is_some() || self.has_composed_message()
    }

    fn has_composed_message(&self) -> bool {
        self.message.is_some()
    }
}

impl<IO> Phase<Sum2, IO>
where
    IO: StateMachineIO,
{
    pub fn new(shared_state: SharedState, io: IO, sum: Sum) -> Self {
        Self {
            shared_state,
            io,
            phase_state: sum.into(),
        }
    }

    async fn fetch_mask_length(mut self) -> Progress<Sum2, IO> {
        if self.phase_state.has_fetched_mask_length() {
            return Progress::Continue(self);
        }

        debug!("polling for mask length");
        match self.io.get_mask_length().await {
            Err(e) => {
                warn!("failed to fetch mask length: {}", e);
                Progress::Stuck(self)
            }
            Ok(None) => {
                debug!("mask length not available yet");
                Progress::Stuck(self)
            }
            Ok(Some(length)) => {
                self.phase_state.mask_length = Some(length);
                Progress::Updated(self.into())
            }
        }
    }

    async fn fetch_seed_dict(mut self) -> Progress<Sum2, IO> {
        if self.phase_state.has_fetched_seed_dict() {
            return Progress::Continue(self);
        }
        debug!("polling for update seeds");
        match self.io.get_seeds(self.shared_state.keys.public).await {
            Err(e) => {
                warn!("failed to fetch seeds: {}", e);
                Progress::Stuck(self)
            }
            Ok(None) => {
                debug!("seeds not available yet");
                Progress::Stuck(self)
            }
            Ok(Some(seeds)) => {
                self.phase_state.seed_dict = Some(seeds);
                Progress::Updated(self.into())
            }
        }
    }

    fn decrypt_seeds(mut self) -> Progress<Sum2, IO> {
        if self.phase_state.has_decrypted_seeds() {
            return Progress::Continue(self);
        }

        let keys = &self.phase_state.ephm_keys;
        let seeds: Result<Vec<MaskSeed>, ()> = self
            .phase_state
            .seed_dict
            .take()
            .unwrap()
            .into_iter()
            .map(|(_, seed)| seed.decrypt(&keys.public, &keys.secret).map_err(|_| ()))
            .collect();

        match seeds {
            Ok(seeds) => {
                self.phase_state.seeds = Some(seeds);
                Progress::Updated(self.into())
            }
            Err(_) => {
                warn!("failed to decrypt mask seeds, going back to waiting phase");
                Progress::Updated(self.into_awaiting().into())
            }
        }
    }

    fn aggregate_masks(mut self) -> Progress<Sum2, IO> {
        if self.phase_state.has_aggregated_masks() {
            return Progress::Continue(self);
        }

        info!("aggregating masks");
        let config = self.shared_state.settings.aggregation.mask;
        let mask_len = self.phase_state.mask_length.unwrap();
        let mask_agg = Aggregation::new(config, config, mask_len as usize);
        for seed in self.phase_state.seeds.take().unwrap().into_iter() {
            let mask = seed.derive_mask(mask_len as usize, config, config);
            if let Err(e) = mask_agg.validate_aggregation(&mask) {
                error!("sum2 phase failed: cannot aggregate masks: {}", e);
                error!("going to awaiting phase");
                return Progress::Updated(self.into_awaiting().into());
            }
        }
        self.phase_state.mask = Some(mask_agg.into());
        Progress::Updated(self.into())
    }

    fn compose_sum2_message(mut self) -> Progress<Sum2, IO> {
        if self.phase_state.has_composed_message() {
            return Progress::Continue(self);
        }

        let sum2 = Sum2Message {
            sum_signature: self.phase_state.sum_signature,
            model_mask: self.phase_state.mask.take().unwrap(),
        };
        self.phase_state.message = Some(self.message_encoder(sum2.into()));
        Progress::Updated(self.into())
    }
}

#[async_trait]
impl<IO> Step<IO> for Phase<Sum2, IO>
where
    IO: StateMachineIO,
{
    async fn step(mut self) -> TransitionOutcome<IO> {
        info!("sum2 task");
        self = try_progress!(self.fetch_mask_length().await);
        self = try_progress!(self.fetch_seed_dict().await);
        self = try_progress!(self.decrypt_seeds());
        self = try_progress!(self.aggregate_masks());
        self = try_progress!(self.compose_sum2_message());

        // FIXME: currently if sending fails, we lose the message,
        // thus wasting all the work we've done in this phase
        let message = self.phase_state.message.take().unwrap();
        match self.send_message(message).await {
            Ok(_) => {
                info!("sent sum2 message");
            }
            Err(e) => {
                warn!("failed to send sum2 message: {}", e);
                warn!("sum2 phase failed");
            }
        }

        info!("going back to awaiting phase");
        TransitionOutcome::Complete(self.into_awaiting().into())
    }
}
