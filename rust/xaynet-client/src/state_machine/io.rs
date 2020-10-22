use xaynet_core::{
    common::RoundParameters,
    mask::Model,
    SumDict,
    SumParticipantPublicKey,
    UpdateSeedDict,
};

pub trait StateMachineIO: CoordinatorClient + ModelStore + Send {}

#[async_trait]
pub trait ModelStore {
    type Error: ::std::fmt::Debug + ::std::error::Error + 'static;

    async fn load_model(&mut self) -> Result<Option<Model>, Self::Error>;
}

/// An interface that API clients implement
#[async_trait]
pub trait CoordinatorClient {
    type Error: ::std::fmt::Debug + ::std::error::Error + 'static;

    /// Retrieve the current round parameters
    async fn get_round_params(&mut self) -> Result<RoundParameters, Self::Error>;

    /// Retrieve the current sum dictionary, if available
    async fn get_sums(&mut self) -> Result<Option<SumDict>, Self::Error>;

    /// Retrieve the current seed dictionary for the given sum
    /// participant, if available.
    async fn get_seeds(
        &mut self,
        pk: SumParticipantPublicKey,
    ) -> Result<Option<UpdateSeedDict>, Self::Error>;

    /// Retrieve the current model/mask length, if available
    async fn get_mask_length(&mut self) -> Result<Option<u64>, Self::Error>;

    /// Retrieve the current global model, if available.
    async fn get_model(&mut self) -> Result<Option<Model>, Self::Error>;

    /// Send an encrypted and signed PET message to the coordinator.
    async fn send_message(&mut self, msg: Vec<u8>) -> Result<(), Self::Error>;
}

pub struct PhaseIO<T, U> {
    coordinator: T,
    model_store: U,
}

impl<T, U> PhaseIO<T, U> {
    pub fn new(coordinator: T, model_store: U) -> Self {
        Self {
            coordinator,
            model_store,
        }
    }
}

#[async_trait]
impl<T, U> CoordinatorClient for PhaseIO<T, U>
where
    T: CoordinatorClient + Send + 'static,
    U: Send + 'static,
{
    type Error = <T as CoordinatorClient>::Error;

    async fn get_round_params(&mut self) -> Result<RoundParameters, Self::Error> {
        self.coordinator.get_round_params().await
    }

    async fn get_sums(&mut self) -> Result<Option<SumDict>, Self::Error> {
        self.coordinator.get_sums().await
    }

    async fn get_seeds(
        &mut self,
        pk: SumParticipantPublicKey,
    ) -> Result<Option<UpdateSeedDict>, Self::Error> {
        self.coordinator.get_seeds(pk).await
    }

    async fn get_mask_length(&mut self) -> Result<Option<u64>, Self::Error> {
        self.coordinator.get_mask_length().await
    }

    async fn get_model(&mut self) -> Result<Option<Model>, Self::Error> {
        self.coordinator.get_model().await
    }

    async fn send_message(&mut self, msg: Vec<u8>) -> Result<(), Self::Error> {
        self.coordinator.send_message(msg).await
    }
}

#[async_trait]
impl<T, U> ModelStore for PhaseIO<T, U>
where
    T: Send + 'static,
    U: ModelStore + Send + 'static,
{
    type Error = <U as ModelStore>::Error;

    async fn load_model(&mut self) -> Result<Option<Model>, Self::Error> {
        self.model_store.load_model().await
    }
}
