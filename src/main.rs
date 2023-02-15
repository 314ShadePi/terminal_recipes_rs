mod cmd;
mod recipes;

use crate::cmd::Cmd;
use home::home_dir;
use inquire::Text;
use std::fs::create_dir;
use std::str::FromStr;

fn main() {
    let home = match home_dir() {
        None => {
            println!("Couldn't get home_dir.");
            std::process::exit(1);
        }
        Some(h) => h,
    };

    let recipe_dir = home.join(".recipes/");

    match recipe_dir.try_exists() {
        Ok(res) => {
            if !res {
                match create_dir(recipe_dir) {
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
