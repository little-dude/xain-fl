use std::error::Error;

use xaynet_core::{
    common::RoundParameters,
    mask::Model,
    SumDict,
    SumParticipantPublicKey,
    UpdateSeedDict,
};

use crate::{ModelStore, Notify, XaynetClient};

pub(crate) fn boxed_io<X, M, N>(
    xaynet_client: X,
    model_store: M,
    notifier: N,
) -> Box<dyn IO<Model = Box<dyn AsRef<Model> + Send>>>
where
    X: XaynetClient + Send + 'static,
    M: ModelStore + Send + 'static,
    N: Notify + Send + 'static,
{
    Box::new((xaynet_client, model_store, notifier))
}

pub struct DefaultNotifier;
impl Notify for DefaultNotifier {}

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

    fn notify_new_round(&mut self);
    fn notify_sum(&mut self);
    fn notify_update(&mut self);
    fn notify_idle(&mut self);
}

#[async_trait]
impl<X, M, N> IO for (X, M, N)
where
    X: XaynetClient + Send + 'static,
    M: ModelStore + Send + 'static,
    N: Notify + Send + 'static,
{
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

    fn notify_new_round(&mut self) {
        self.2.notify_new_round()
    }

    fn notify_sum(&mut self) {
        self.2.notify_sum()
    }

    fn notify_update(&mut self) {
        self.2.notify_update()
    }

    fn notify_idle(&mut self) {
        self.2.notify_idle()
    }
}

#[async_trait]
impl IO for Box<dyn IO<Model = Box<dyn AsRef<Model> + Send>>> {
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

    fn notify_new_round(&mut self) {
        self.as_mut().notify_new_round()
    }

    fn notify_sum(&mut self) {
        self.as_mut().notify_sum()
    }

    fn notify_update(&mut self) {
        self.as_mut().notify_update()
    }

    fn notify_idle(&mut self) {
        self.as_mut().notify_idle()
    }
}
