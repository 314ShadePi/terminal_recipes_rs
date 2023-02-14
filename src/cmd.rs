use std::str::FromStr;

pub enum Cmd {
    Exit,
}

impl FromStr for Cmd {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "exit" => Ok(Self::Exit),
            _ => Err(()),
        }
    }
}
