use xaynet_core::{crypto::EncryptKeyPair, mask::MaskConfig};

mod max_message_size;
pub use max_message_size::MaxMessageSize;

#[derive(Serialize, Deserialize, Debug)]
pub struct AggregationConfig {
    pub mask: MaskConfig,
    pub scalar: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub keys: EncryptKeyPair,
    pub aggregation: AggregationConfig,
    pub max_message_size: MaxMessageSize,
}
