#![deny(clippy::pedantic, clippy::complexity)]

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
        Path::new(&<&str>::clone(&RECIPE_DIR))
            .join("data/")
            .to_str()
            .unwrap()
            .to_string()
            .to_static_str()
    };
}

lazy_static! {
    pub static ref CACHE_FILE: &'static str = {
        Path::new(&<&str>::clone(&DATA_DIR))
            .join("cache.json")
            .to_str()
            .unwrap()
            .to_string()
            .to_static_str()
    };
}

lazy_static! {
    pub static ref CONFIG_FILE: &'static str = {
        Path::new(&<&str>::clone(&DATA_DIR))
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
        &RECIPE_DIR.to_string(),
        &DATA_DIR.to_string(),
        &CACHE_FILE.to_string(),
        &CONFIG_FILE.to_string()
    );

    initializer::init().unwrap();

    let error_handler = |e: anyhow::Error| {
        println!("ERROR: {e}");
        Ok(())
    };

    cl::CommandLine::command_line("Terminal Recipes> ", error_handler);
}
