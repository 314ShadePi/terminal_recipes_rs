mod cache;
mod cfg;
mod cmd;
mod initializer;
mod recipe;

use crate::cache::Cache;
use crate::cfg::Config;
use crate::cmd::Cmd;
use crate::initializer::init;
use crate::recipe::Recipe;
use inquire::{validator::Validation, Text};
use std::fs::read_to_string;
use std::path::PathBuf;
use std::str::FromStr;
use strum::VariantNames;

fn main() {
    let (recipe_dir, cfg) = init();

    let cmd_validator = |input: &str| {
        let s: (&str, &str) = match input.contains(" ") {
            true => match input.split_once(' ') {
                None => return Ok(Validation::Invalid("Couldn't parse command.".into())),
                Some(s) => s,
            },
            false => (input, ""),
        };

        if Cmd::VARIANTS.contains(&s.0) {
            Ok(Validation::Valid)
        } else {
            Ok(Validation::Invalid("Not a command.".into()))
        }
    };

    loop {
        let recipe_dir = recipe_dir.clone();
        let cfg = cfg.clone();
        let cmd = Text::new("Terminal Recipes> ")
            .with_validator(cmd_validator)
            .prompt();

        match cmd {
            Ok(cmd_str) => handle_cmd(cmd_str, recipe_dir, cfg),
            Err(_) => {}
        }
    }
}

fn handle_cmd(cmd: String, recipe_dir: PathBuf, cfg: (PathBuf, Config)) {
    let cmd = Cmd::from_str(cmd.as_str()).unwrap();

    match cmd {
        Cmd::Exit => std::process::exit(0),
        Cmd::List => list(recipe_dir.clone(), cfg.1.clone()),
        Cmd::View(recipe) => Recipe::view(
            recipe_dir.clone().join("data/").join("cache.json"),
            recipe_dir.clone(),
            recipe.clone(),
        ),
        Cmd::Config(_) => {}
        Cmd::RebuildCache => {
            println!("Rebuilding cache, please wait...");
            Cache::rebuild_cache(recipe_dir.clone().join("data/"), recipe_dir.clone());
            println!("Cache rebuilt!");
        }
    }
}

fn list(recipe_dir: PathBuf, _cfg: Config) {
    let cache = recipe_dir.clone().join("data/").join("cache.json");
    let file_content = match read_to_string(cache.clone()) {
        Ok(f) => f,
        Err(e) => {
            format!("ERROR: {}", e)
        }
    };

    let file = match serde_json::from_str::<Cache>(&file_content) {
        Ok(c) => c,
        Err(_) => return,
    };

    print!("{}", file);
}
