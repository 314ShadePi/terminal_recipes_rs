mod cache;
mod cfg;
mod cmd;
mod initializer;
mod recipe;

use crate::cmd::Cmd;
use crate::initializer::init;
use crate::recipe::{Ingredient, Recipe};
use inquire::Text;
use std::str::FromStr;

fn main() {
    /*let recipe = Recipe {
        name: "test".to_string(),
        ingredients: vec![
            Ingredient {
                amount: "123".to_string(),
                name: "test".to_string(),
            },
            Ingredient {
                amount: "123".to_string(),
                name: "test".to_string(),
            },
        ],
        steps: Some(vec!["step".to_string(), "step".to_string()]),
    };

    println!("{}", serde_json::to_string_pretty(&recipe).unwrap());*/

    let (recipe_dir, (cfg_file, cfg)) = init();

    loop {
        let cmd = Text::new("Terminal Recipes> ").prompt();

        match cmd {
            Ok(cmd_str) => handle_cmd(cmd_str),
            Err(_) => {}
        }
    }
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
