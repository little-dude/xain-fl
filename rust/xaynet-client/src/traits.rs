use std::error::Error;

use xaynet_core::{
    common::RoundParameters,
    mask::Model,
    SumDict,
    SumParticipantPublicKey,
    UpdateSeedDict,
};

#[async_trait]
pub(crate) trait IO: Send + 'static {
    async fn load_model(&mut self) -> Result<Option<Model>, Box<dyn Error>>;
    async fn get_round_params(&mut self) -> Result<RoundParameters, Box<dyn Error>>;
    async fn get_sums(&mut self) -> Result<Option<SumDict>, Box<dyn Error>>;
    async fn get_seeds(
        &mut self,
        pk: SumParticipantPublicKey,
    ) -> Result<Option<UpdateSeedDict>, Box<dyn Error>>;
    async fn get_mask_length(&mut self) -> Result<Option<u64>, Box<dyn Error>>;
    async fn get_model(&mut self) -> Result<Option<Model>, Box<dyn Error>>;
    async fn send_message(&mut self, msg: Vec<u8>) -> Result<(), Box<dyn Error>>;
}

#[async_trait]
pub trait ModelStore {
    type Error: ::std::fmt::Debug + ::std::error::Error + 'static;

    async fn load_model(&mut self) -> Result<Option<Model>, Box<dyn Error>>;
}

/// An interface that API clients implement
#[async_trait]
pub trait XaynetClient {
    type Error: ::std::fmt::Debug + ::std::error::Error + 'static;

    /// Retrieve the current round parameters
    async fn get_round_params(&mut self) -> Result<RoundParameters, Box<dyn Error>>;

    /// Retrieve the current sum dictionary, if available
    async fn get_sums(&mut self) -> Result<Option<SumDict>, Box<dyn Error>>;

    /// Retrieve the current seed dictionary for the given sum
    /// participant, if available.
    async fn get_seeds(
        &mut self,
        pk: SumParticipantPublicKey,
    ) -> Result<Option<UpdateSeedDict>, Box<dyn Error>>;

    /// Retrieve the current model/mask length, if available
    async fn get_mask_length(&mut self) -> Result<Option<u64>, Box<dyn Error>>;

    /// Retrieve the current global model, if available.
    async fn get_model(&mut self) -> Result<Option<Model>, Box<dyn Error>>;

    /// Send an encrypted and signed PET message to the coordinator.
    async fn send_message(&mut self, msg: Vec<u8>) -> Result<(), Box<dyn Error>>;
}

struct ParticipantIo<T, U> {
    xaynet_client: T,
    model_store: U,
}

impl<T, U> ParticipantIo<T, U> {
    pub fn new(xaynet_client: T, model_store: U) -> Self {
        Self {
            xaynet_client,
            model_store,
        }
    }
}

pub(crate) fn boxed_io<T, U>(xaynet_client: T, model_store: U) -> Box<dyn IO>
where
    T: XaynetClient + Send + 'static,
    U: ModelStore + Send + 'static,
{
    Box::new(ParticipantIo::new(xaynet_client, model_store))
}

#[async_trait]
impl<T, U> IO for ParticipantIo<T, U>
where
    T: XaynetClient + Send + 'static,
    U: ModelStore + Send + 'static,
{
    async fn get_round_params(&mut self) -> Result<RoundParameters, Box<dyn Error>> {
        self.xaynet_client.get_round_params().await
    }

    async fn get_sums(&mut self) -> Result<Option<SumDict>, Box<dyn Error>> {
        self.xaynet_client.get_sums().await
    }

    async fn get_seeds(
        &mut self,
        pk: SumParticipantPublicKey,
    ) -> Result<Option<UpdateSeedDict>, Box<dyn Error>> {
        self.xaynet_client.get_seeds(pk).await
    }

    async fn get_mask_length(&mut self) -> Result<Option<u64>, Box<dyn Error>> {
        self.xaynet_client.get_mask_length().await
    }

    async fn get_model(&mut self) -> Result<Option<Model>, Box<dyn Error>> {
        self.xaynet_client.get_model().await
    }

    async fn send_message(&mut self, msg: Vec<u8>) -> Result<(), Box<dyn Error>> {
        self.xaynet_client.send_message(msg).await
    }

    async fn load_model(&mut self) -> Result<Option<Model>, Box<dyn Error>> {
        self.model_store.load_model().await
    }
}

#[async_trait]
impl IO for Box<dyn IO> {
    async fn get_round_params(&mut self) -> Result<RoundParameters, Box<dyn Error>> {
        self.as_mut().get_round_params().await
    }

    async fn get_sums(&mut self) -> Result<Option<SumDict>, Box<dyn Error>> {
        self.as_mut().get_sums().await
    }

    async fn get_seeds(
        &mut self,
        pk: SumParticipantPublicKey,
    ) -> Result<Option<UpdateSeedDict>, Box<dyn Error>> {
        self.as_mut().get_seeds(pk).await
    }

    async fn get_mask_length(&mut self) -> Result<Option<u64>, Box<dyn Error>> {
        self.as_mut().get_mask_length().await
    }

    async fn get_model(&mut self) -> Result<Option<Model>, Box<dyn Error>> {
        self.as_mut().get_model().await
    }

    async fn send_message(&mut self, msg: Vec<u8>) -> Result<(), Box<dyn Error>> {
        self.as_mut().send_message(msg).await
    }

    async fn load_model(&mut self) -> Result<Option<Model>, Box<dyn Error>> {
        self.as_mut().load_model().await
    }
}
