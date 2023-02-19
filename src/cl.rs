use crate::commands::list::List;
use cmd_sys::EnumCommandLine;

#[derive(Clone, EnumCommandLine)]
pub enum CommandLine {
    List(List),
}
