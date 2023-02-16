use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Cache {
    pub entries: Vec<CacheEntry>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct CacheEntry {
    pub name: String,
    pub path: PathBuf,
}
