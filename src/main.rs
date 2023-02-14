mod cmd;
mod recipes;

use crate::cmd::Cmd;
use inquire::Text;
use std::str::FromStr;

fn main() {
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
    }
}
