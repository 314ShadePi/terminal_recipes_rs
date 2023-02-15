use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
pub enum Cmd {
    Exit,
    List,
    View(String),
}

impl FromStr for Cmd {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = match s.contains(" ") {
            true => match s.split_once(' ') {
                None => return Err(()),
                Some(s) => s,
            },
            false => (s, ""),
        };

        match s {
            ("exit", _) => Ok(Self::Exit),
            ("list", _) => Ok(Self::List),
            ("view", param) => Ok(Self::View(param.to_string())),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cmd_from_str_exit() {
        let exit = Cmd::from_str("exit").unwrap();
        assert_eq!(exit, Cmd::Exit);
    }

    #[test]
    fn cmd_from_str_list() {
        let list = Cmd::from_str("list").unwrap();
        assert_eq!(list, Cmd::List);
    }

    #[test]
    fn cmd_from_str_view_no_params() {
        let view_1 = Cmd::from_str("view").unwrap();
        assert_eq!(view_1, Cmd::View("".to_string()));
    }

    #[test]
    fn cmd_from_str_view_one_param() {
        let view_2 = Cmd::from_str("view qwert").unwrap();
        assert_eq!(view_2, Cmd::View("qwert".to_string()));
    }

    #[test]
    fn cmd_from_str_view_two_params() {
        let view_3 = Cmd::from_str("view qwert yuiop").unwrap();
        assert_eq!(view_3, Cmd::View("qwert yuiop".to_string()));
    }
}
