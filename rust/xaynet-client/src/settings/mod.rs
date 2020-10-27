use xaynet_core::{crypto::SigningKeyPair, mask::MaskConfig};

mod max_message_size;
pub use max_message_size::MaxMessageSize;

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub keys: SigningKeyPair,
    pub mask_config: MaskConfig,
    pub scalar: f64,
    pub max_message_size: MaxMessageSize,
}
