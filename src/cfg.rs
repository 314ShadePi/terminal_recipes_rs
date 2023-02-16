use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub rebuild_cache_on_startup: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            rebuild_cache_on_startup: true,
        }
    }
}
