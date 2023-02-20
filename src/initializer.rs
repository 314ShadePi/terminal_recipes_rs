use crate::cache::Cache;
use crate::config::Config;
use crate::{CONFIG_FILE, DATA_DIR, RECIPE_DIR};
use std::fs::{create_dir, read_to_string, write, File};
use std::path::PathBuf;

pub fn init() -> Result<(), ()> {
    create_dir_c(PathBuf::from(&RECIPE_DIR.to_string()))?;
    create_dir_c(PathBuf::from(&DATA_DIR.to_string()))?;
    let config = load_config(true)?;

    if config.rebuild_cache_on_startup {
        Cache::rebuild()?;
    }

    Ok(())
}

fn create_dir_c(path: PathBuf) -> Result<(), ()> {
    match path.clone().try_exists() {
        Ok(res) => {
            if !res {
                return match create_dir(path) {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        println!("ERROR: {e}");
                        Err(())
                    }
                };
            }
        }
        Err(e) => {
            println!("ERROR: {e}");
            return Err(());
        }
    }

    Ok(())
}

pub fn load_config(first: bool) -> Result<Config, ()> {
    let exec = |p: PathBuf| {
        let content = match serde_json::to_string_pretty(&Config::default()) {
            Ok(c) => c,
            Err(e) => {
                println!("ERROR: {e}");
                return Err(());
            }
        };
        return match write(p, content) {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("ERROR: {e}");
                Err(())
            }
        };
    };

    create_file_c(PathBuf::from(&CONFIG_FILE.to_string()), exec)?;

    let cfg_content = match read_to_string(PathBuf::from(&CONFIG_FILE.to_string())) {
        Ok(f) => f,
        Err(e) => {
            println!("ERROR: {e}");
            return Err(());
        }
    };

    let cfg_data = match serde_json::from_str::<Config>(&cfg_content) {
        Ok(cfg) => cfg,
        Err(e) => {
            println!("ERROR: {e}");
            if first {
                exec(PathBuf::from(&CONFIG_FILE.to_string()))?;
                load_config(false)?
            } else {
                return Err(());
            }
        }
    };

    Ok(cfg_data)
}

pub fn create_file_c<F>(path: PathBuf, exec: F) -> Result<(), ()>
where
    F: Fn(PathBuf) -> Result<(), ()>,
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
                        println!("ERROR: {e}");
                        Err(())
                    }
                };
            }
        }
        Err(e) => {
            println!("ERROR: {e}");
            return Err(());
        }
    }

    Ok(())
}
