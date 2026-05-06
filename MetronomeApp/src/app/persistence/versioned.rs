use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct VersionedConfig<T> {
    pub version: u32,
    pub data: T,
}
