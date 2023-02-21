#![allow(clippy::cmp_owned)]

use crate::commands::list::List;
use crate::commands::rebuild_cache::RebuildCache;
use crate::commands::view::View;
use cmd_sys::EnumCommandLine;
use crate::commands::config::ConfigCmd;

#[derive(Debug, Clone, EnumCommandLine)]
pub enum CommandLine {
    List(List),
    View(View),
    RebuildCache(RebuildCache),
    ConfigCmd(ConfigCmd),
}
