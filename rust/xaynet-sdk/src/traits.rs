use xaynet_core::{
    common::RoundParameters,
    mask::Model,
    SumDict,
    SumParticipantPublicKey,
    UpdateSeedDict,
};

pub trait Notify {
    fn notify_new_round(&mut self) {}
    fn notify_sum(&mut self) {}
    fn notify_update(&mut self) {}
    fn notify_idle(&mut self) {}
}

#[async_trait]
pub trait ModelStore {
    type Error: ::std::error::Error;
    type Model: AsRef<Model> + Send;

    async fn load_model(&mut self) -> Result<Option<Self::Model>, Self::Error>;
}

/// An interface that API clients implement
#[async_trait]
pub trait XaynetClient {
    type Error: ::std::error::Error;

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
