pub use notation_model::prelude::*;

pub type LoadTabError = ron::error::SpannedError;
pub type LoadTabResult = anyhow::Result<ProtoTab, LoadTabError>;

pub fn load_tab(bytes: &Vec<u8>) -> LoadTabResult {
    ron::de::from_bytes::<ProtoTab>(bytes)
}