use anyhow::{bail, Context};
use cmd_sys::Command;
use terminal_recipes_rs_lib::ConfigOptions;
use crate::config::Config;

#[derive(Debug, Clone)]
pub struct ConfigCmd {
    option: String,
    value: String,
}

impl Command for ConfigCmd {
    const CMD: &'static str = "config";

    #[tracing::instrument]
    fn run(&self) -> anyhow::Result<()> {
        let config = Config::get_config(true)?;
        if !Config::get_opts().contains(&self.option) {
            tracing::error!("Not a valid option: {}", self.option);
            bail!("Not a valid option: {}", self.option);
        }
        let config = if self.option == "rebuild-cache-on-startup" {
            Config {
                rebuild_cache_on_startup: self.value.parse().context("Couldn't parse config value:")?,
                ..config
            }
        } else { config };

        Config::write_config(config)
    }

    #[tracing::instrument]
    fn new_cmd(params: Vec<String>) -> anyhow::Result<Self> {
        if params.len() < 2 {
            tracing::error!("Not enough parameters.");
            bail!("Not enough parameters.")
        }
        if params.len() > 2 {
            tracing::warn!("Too many parameters, discarding from after params[1].");
            println!("Too many parameters, discarding from after params[1].");
        }
        Ok(Self { option: params[0].clone(), value: params[1].clone() })
    }
}