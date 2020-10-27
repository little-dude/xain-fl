use std::error::Error;

use xaynet_core::{
    common::RoundParameters,
    mask::Model,
    SumDict,
    SumParticipantPublicKey,
    UpdateSeedDict,
};

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
