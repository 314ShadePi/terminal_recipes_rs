#![deny(clippy::pedantic, clippy::complexity)]

use c314_utils::prelude::ToStaticStr;
use cmd_sys::EnumCommandLine;
use home::home_dir;
use lazy_static::lazy_static;
use std::clone::Clone;
use std::path::{Path, PathBuf};
use std::string::ToString;
use tracing::Level;
use tracing_subscriber::fmt;
use tracing_subscriber::fmt::format::FmtSpan;

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
    let file_appender = tracing_appender::rolling::hourly(
        PathBuf::from(&<&str>::clone(&DATA_DIR)),
        "terminal_recipes_rs.log",
    );

    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let subscriber = fmt()
        .with_writer(non_blocking)
        .with_max_level(Level::TRACE)
        .with_file(true)
        .with_line_number(true)
        .with_span_events(FmtSpan::FULL)
        .json()
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    tracing::trace!(
        "{}::::{}::::{}::::{}",
        &RECIPE_DIR.to_string(),
        &DATA_DIR.to_string(),
        &CACHE_FILE.to_string(),
        &CONFIG_FILE.to_string()
    );

    initializer::init().unwrap();

    let error_handler = |e: anyhow::Error| {
        eprintln!("{e:#}");
        Ok(())
    };

    cl::CommandLine::command_line("Terminal Recipes> ", error_handler);
}
