use xaynet_core::{crypto::SigningKeyPair, mask::MaskConfig};

mod max_message_size;
pub use max_message_size::MaxMessageSize;

#[derive(Serialize, Deserialize, Debug)]
pub struct AgentSettings {
    pub(crate) keys: SigningKeyPair,
    pub(crate) mask_config: MaskConfig,
    pub(crate) scalar: f64,
    pub(crate) max_message_size: MaxMessageSize,
}

impl AgentSettings {
    pub fn new(keys: SigningKeyPair, mask_config: MaskConfig) -> Self {
        AgentSettings {
            keys,
            mask_config,
            scalar: 1.0,
            max_message_size: MaxMessageSize::default(),
        }
    }
}
