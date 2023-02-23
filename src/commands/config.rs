use crate::config::Config;
use crate::CONFIG_FILE;
use anyhow::bail;
use cmd_sys::Command;
use terminal_recipes_rs_lib::ConfigOptions;

#[derive(Debug, Clone)]
pub struct ConfigCmd {
    option: String,
    value: String,
}

impl Command for ConfigCmd {
    const CMD: &'static str = "config";
    const HELP_SHORT: &'static str = "config <option> <value> -- Change config.";
    const HELP_LONG: &'static str = "config <option> <value> -- Change config.";

    #[tracing::instrument(name = "ConfigCmd::run()")]
    fn run(&self) -> anyhow::Result<()> {
        Config::get_config(true)?
            .update_cfg(&self.option, &self.value)?
            .write_cfg(<&str>::clone(&CONFIG_FILE))
    }

    #[tracing::instrument(name = "ConfigCmd::new_cmd()")]
    fn new_cmd(params: Vec<String>) -> anyhow::Result<Self> {
        if params.len() < 2 {
            tracing::error!("Not enough parameters.");
            bail!("Not enough parameters.")
        }
        if params.len() > 2 {
            tracing::warn!("Too many parameters, discarding from after params[1].");
            println!("Too many parameters, discarding from after params[1].");
        }
        Ok(Self {
            option: params[0].clone(),
            value: params[1].clone(),
        })
    }
}
