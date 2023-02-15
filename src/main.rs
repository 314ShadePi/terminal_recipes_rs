mod cfg;
mod cmd;
mod recipes;

use crate::cfg::Config;
use crate::cmd::Cmd;
use home::home_dir;
use inquire::Text;
use std::fs::{create_dir, read_to_string, write, File};
use std::path::PathBuf;
use std::str::FromStr;

fn main() {
    let recipe_dir = get_recipe_dir();
    create_data_dir(recipe_dir.clone());
    let (cfg_file, cfg) = load_config(recipe_dir.clone());

    loop {
        let cmd = Text::new("Terminal Recipes> ").prompt();

        match cmd {
            Ok(cmd_str) => handle_cmd(cmd_str),
            Err(_) => {}
        }
    }
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

    match recipe_dir.clone().try_exists() {
        Ok(res) => {
            if !res {
                match create_dir(recipe_dir.clone()) {
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

    recipe_dir
}

fn create_data_dir(recipe_dir: PathBuf) {
    let data_dir = recipe_dir.join("data/");

    match data_dir.clone().try_exists() {
        Ok(res) => {
            if !res {
                match create_dir(data_dir.clone()) {
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

fn handle_cmd(cmd: String) {
    let cmd = Cmd::from_str(cmd.as_str());
    let cmd = match cmd {
        Ok(cmd) => cmd,
        Err(_) => {
            eprintln!("Couldn't parse command, please try again");
            return;
        }
    };

    match cmd {
        Cmd::Exit => std::process::exit(0),
        Cmd::List => {}
        Cmd::View(_) => {}
    }
}
