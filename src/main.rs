mod cache;
mod cfg;
mod cmd;
mod initializer;
mod recipe;

use crate::cmd::Cmd;
use crate::initializer::init;
use inquire::{validator::Validation, Text};
use std::str::FromStr;
use strum::VariantNames;

fn main() {
    let (_recipe_dir, (_cfg_file, _cfg)) = init();

    let cmd_validator = |input: &str| {
        if Cmd::VARIANTS.contains(&input) {
            Ok(Validation::Valid)
        } else {
            Ok(Validation::Invalid("Not a command.".into()))
        }
    };

    loop {
        let cmd = Text::new("Terminal Recipes> ")
            .with_validator(cmd_validator)
            .prompt();

        match cmd {
            Ok(cmd_str) => handle_cmd(cmd_str),
            Err(_) => {}
        }
    }
}

fn handle_cmd(cmd: String) {
    let cmd = Cmd::from_str(cmd.as_str()).unwrap();

    match cmd {
        Cmd::Exit => std::process::exit(0),
        Cmd::List => {}
        Cmd::View(_) => {}
        Cmd::Config(_) => {}
    }
}
