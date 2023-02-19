use c314_utils::prelude::ToStaticStr;
use cmd_sys::EnumCommandLine;
use home::home_dir;
use lazy_static::lazy_static;
use std::clone::Clone;
use std::path::Path;
use std::string::ToString;

mod cache;
mod cl;
mod commands;
mod config;
mod initializer;
mod recipe;

lazy_static! {
    pub static ref RECIPE_DIR: &'static str = {
        home_dir()
            .unwrap()
            .join(".recipes/")
            .to_str()
            .unwrap()
            .to_string()
            .to_static_str()
    };
}

lazy_static! {
    pub static ref DATA_DIR: &'static str = {
        Path::new(&RECIPE_DIR.clone())
            .join("data/")
            .to_str()
            .unwrap()
            .to_string()
            .to_static_str()
    };
}

lazy_static! {
    pub static ref CACHE_FILE: &'static str = {
        Path::new(&DATA_DIR.clone())
            .join("cache.json")
            .to_str()
            .unwrap()
            .to_string()
            .to_static_str()
    };
}

lazy_static! {
    pub static ref CONFIG_FILE: &'static str = {
        Path::new(&DATA_DIR.clone())
            .join("config.json")
            .to_str()
            .unwrap()
            .to_string()
            .to_static_str()
    };
}

fn main() {
    println!(
        "{}::::{}::::{}::::{}",
        &RECIPE_DIR.clone().to_string(),
        &DATA_DIR.clone().to_string(),
        &CACHE_FILE.clone().to_string(),
        &CONFIG_FILE.clone().to_string()
    );

    initializer::init().unwrap();
    cl::CommandLine::command_line("Terminal Recipes> ")
}
