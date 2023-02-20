#![allow(clippy::cmp_owned)]

use crate::commands::list::List;
use crate::commands::rebuild_cache::RebuildCache;
use crate::commands::view::View;
use cmd_sys::EnumCommandLine;

#[derive(Clone, EnumCommandLine)]
pub enum CommandLine {
    List(List),
    View(View),
    RebuildCache(RebuildCache),
}
