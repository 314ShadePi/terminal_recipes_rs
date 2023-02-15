use crate::cfg::Config;
use home::home_dir;
use std::fs::{create_dir, read_to_string, write, File};
use std::path::PathBuf;
use std::str::FromStr;

pub fn init() -> (PathBuf, (PathBuf, Config)) {
    let recipe_dir = get_recipe_dir();
    create_data_dir(recipe_dir.clone());
    let cfg = load_config(recipe_dir.clone());

    (recipe_dir, cfg)
}

fn get_recipe_dir() -> PathBuf {
    let home = match home_dir() {
        None => {
            println!("Couldn't get home_dir.");
            std::process::exit(1);
        }
        Some(h) => h,
    };

    let recipe_dir = home.join(".recipes/");

    create_dir_c(recipe_dir.clone());

    recipe_dir
}

fn create_data_dir(recipe_dir: PathBuf) {
    let data_dir = recipe_dir.join("data/");

    create_dir_c(data_dir);
}

fn load_config(recipe_dir: PathBuf) -> (PathBuf, Config) {
    let cfg_path = recipe_dir.clone().join("data/").join("config.json");

    match cfg_path.clone().try_exists() {
        Ok(res) => {
            if !res {
                match File::create(cfg_path.clone()) {
                    Ok(_) => {
                        let content = match serde_json::to_string_pretty(&Config::default()) {
                            Ok(c) => c,
                            Err(e) => {
                                println!("ERROR: {}", e);
                                std::process::exit(1);
                            }
                        };
                        match write(cfg_path.clone(), content) {
                            Ok(_) => {}
                            Err(e) => {
                                println!("ERROR: {}", e);
                                std::process::exit(1);
                            }
                        }
                    }
                    Err(e) => {
                        println!("ERROR: {}", e);
                        std::process::exit(1);
                    }
                }
            }
        }
        Err(e) => {
            println!("ERROR: {}", e);
            std::process::exit(1);
        }
    }

    let cfg_content = match read_to_string(cfg_path.clone()) {
        Ok(f) => f,
        Err(e) => {
            println!("ERROR: {}", e);
            std::process::exit(1);
        }
    };

    let cfg_data = match serde_json::from_str::<Config>(&cfg_content) {
        Ok(cfg) => cfg,
        Err(e) => {
            println!("ERROR: {}", e);
            std::process::exit(1);
        }
    };

    (cfg_path, cfg_data)
}

fn create_dir_c(dir: PathBuf) {
    match dir.clone().try_exists() {
        Ok(res) => {
            if !res {
                match create_dir(dir.clone()) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("ERROR: {}", e);
                        std::process::exit(1);
                    }
                }
            }
        }
        Err(e) => {
            println!("ERROR: {}", e);
            std::process::exit(1);
        }
    }
}
