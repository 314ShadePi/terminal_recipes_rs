use std::fs::{read_to_string, write};
use anyhow::{anyhow, Context};
use serde::{Deserialize, Serialize};
use terminal_recipes_rs_lib::ConfigOptions;
use crate::CONFIG_FILE;

#[derive(Debug, Clone, Deserialize, Serialize, ConfigOptions)]
pub struct Config {
    pub rebuild_cache_on_startup: bool,
}

impl Config {
    #[tracing::instrument]
    pub fn get_config(first: bool) -> anyhow::Result<Self> {
        let config = read_to_string(<&str>::clone(&CONFIG_FILE)).context("Couldn't read cache.")?;

        let config = match serde_json::from_str::<Config>(&config) {
            Ok(c) => c,
            Err(e) => {
                if first {
                    let content = serde_json::to_string_pretty(&Config::default())
                        .context("Couldn't serialize config.")?;
                    write(<&str>::clone(&CONFIG_FILE), content).context("Couldn't write config.")?;
                    Self::get_config(false)?
                } else {
                    let e = anyhow!(e);
                    let e = e.context("Couldn't deserialize cache.");
                    return Err(e);
                }
            }
        };

        Ok(config)
    }

    pub fn write_config(config: Self) -> anyhow::Result<()> {
        let content = serde_json::to_string_pretty(&config).context("Couldn't serialize config.")?;
        write(<&str>::clone(&CONFIG_FILE), content).context("Couldn't write config.")
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            rebuild_cache_on_startup: true,
        }
    }
}
