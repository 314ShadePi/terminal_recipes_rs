use crate::cache::Cache;
use crate::config::Config;
use crate::{CONFIG_FILE, DATA_DIR, RECIPE_DIR};
use anyhow::{anyhow, Context};
use std::fs::{create_dir, read_to_string, write, File};
use std::path::PathBuf;

#[tracing::instrument]
pub fn init() -> anyhow::Result<()> {
    create_dir_c(&PathBuf::from(&RECIPE_DIR.to_string()))?;
    create_dir_c(&PathBuf::from(&DATA_DIR.to_string()))?;
    let config = load_config(true)?;

    if config.rebuild_cache_on_startup {
        Cache::rebuild()?;
    }

    Ok(())
}

#[tracing::instrument]
fn create_dir_c(path: &PathBuf) -> anyhow::Result<()> {
    match path.try_exists() {
        Ok(res) => {
            if !res {
                return match create_dir(path.clone()) {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        let e = anyhow!(e);
                        let error = format!("Couldn't create dir {path:#?}");
                        let e = e.context(error);
                        Err(e)
                    }
                };
            }
        }
        Err(e) => {
            let e = anyhow!(e);
            let e = e.context("Couldn't check if dir exists.");
            return Err(e);
        }
    }

    Ok(())
}

#[tracing::instrument]
pub fn load_config(first: bool) -> anyhow::Result<Config> {
    let exec = |p: PathBuf| {
        let content = serde_json::to_string_pretty(&Config::default())
            .context("Couldn't serialize config.")?;
        write(p, content).context("Couldn't write config.")
    };

    create_file_c(PathBuf::from(&CONFIG_FILE.to_string()), exec)?;

    let cfg_content =
        read_to_string(PathBuf::from(&CONFIG_FILE.to_string())).context("Couldn't read config.")?;

    let cfg_data = match serde_json::from_str::<Config>(&cfg_content) {
        Ok(cfg) => cfg,
        Err(e) => {
            if first {
                exec(PathBuf::from(&CONFIG_FILE.to_string()))?;
                load_config(false)?
            } else {
                let e = anyhow!(e);
                let e = e.context("Couldn't deserialize config.");
                return Err(e);
            }
        }
    };

    Ok(cfg_data)
}

#[tracing::instrument(skip(exec))]
pub fn create_file_c<F>(path: PathBuf, exec: F) -> anyhow::Result<()>
where
    F: Fn(PathBuf) -> anyhow::Result<()>,
{
    match path.clone().try_exists() {
        Ok(res) => {
            if !res {
                return match File::create(path.clone()) {
                    Ok(_) => {
                        exec(path)?;
                        Ok(())
                    }
                    Err(e) => {
                        let e = anyhow!(e);
                        let err = format!("Couldn't create file {path:#?}");
                        let e = e.context(err);
                        Err(e)
                    }
                };
            }
        }
        Err(e) => {
            let e = anyhow!(e);
            let e = e.context("Couldn't check if file exists.");
            return Err(e);
        }
    }

    Ok(())
}
