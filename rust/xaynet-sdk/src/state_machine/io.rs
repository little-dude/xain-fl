use std::error::Error;

use xaynet_core::{
    common::RoundParameters,
    mask::Model,
    SumDict,
    SumParticipantPublicKey,
    UpdateSeedDict,
};

use crate::{ModelStore, XaynetClient};

#[async_trait]
pub(crate) trait IO: Send + 'static {
    type Model;

    async fn load_model(&mut self) -> Result<Option<Self::Model>, Box<dyn Error>>;
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

pub(crate) fn boxed_io<T, U>(
    xaynet_client: T,
    model_store: U,
) -> Box<dyn IO<Model = Box<dyn AsRef<Model> + Send>>>
where
    T: XaynetClient + Send + 'static,
    U: ModelStore + Send + 'static,
{
    Box::new((xaynet_client, model_store))
}

#[async_trait]
impl<T, U> IO for (T, U)
where
    T: XaynetClient + Send + 'static,
    U: ModelStore + Send + 'static,
{
    // type Model = <U as ModelStore>::Model;
    type Model = Box<dyn AsRef<Model> + Send>;

    async fn load_model(&mut self) -> Result<Option<Self::Model>, Box<dyn Error>> {
        self.1
            .load_model()
            .await
            .map_err(|e| Box::new(e) as Box<dyn Error>)
            .map(|opt| opt.map(|model| Box::new(model) as Box<dyn AsRef<Model> + Send>))
    }

    async fn get_round_params(&mut self) -> Result<RoundParameters, Box<dyn Error>> {
        self.0
            .get_round_params()
            .await
            .map_err(|e| Box::new(e) as Box<dyn Error>)
    }

    async fn get_sums(&mut self) -> Result<Option<SumDict>, Box<dyn Error>> {
        self.0
            .get_sums()
            .await
            .map_err(|e| Box::new(e) as Box<dyn Error>)
    }

    async fn get_seeds(
        &mut self,
        pk: SumParticipantPublicKey,
    ) -> Result<Option<UpdateSeedDict>, Box<dyn Error>> {
        self.0
            .get_seeds(pk)
            .await
            .map_err(|e| Box::new(e) as Box<dyn Error>)
    }

    async fn get_mask_length(&mut self) -> Result<Option<u64>, Box<dyn Error>> {
        self.0
            .get_mask_length()
            .await
            .map_err(|e| Box::new(e) as Box<dyn Error>)
    }

    async fn get_model(&mut self) -> Result<Option<Model>, Box<dyn Error>> {
        self.0
            .get_model()
            .await
            .map_err(|e| Box::new(e) as Box<dyn Error>)
    }

    async fn send_message(&mut self, msg: Vec<u8>) -> Result<(), Box<dyn Error>> {
        self.0
            .send_message(msg)
            .await
            .map_err(|e| Box::new(e) as Box<dyn Error>)
    }
}

#[async_trait]
impl IO for Box<dyn IO<Model = Box<dyn AsRef<Model> + Send>>> {
    // type Model = <<Self as std::ops::Deref<dyn IO>>::Target as IO>::Model;
    type Model = Box<dyn AsRef<Model> + Send>;

    async fn load_model(&mut self) -> Result<Option<Self::Model>, Box<dyn Error>> {
        self.as_mut().load_model().await
    }

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
}
