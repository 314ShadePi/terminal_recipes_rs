use crate::commands::list::List;
use crate::commands::rebuild_cache::RebuildCache;
use cmd_sys::EnumCommandLine;

#[derive(Clone, EnumCommandLine)]
pub enum CommandLine {
    List(List),
    RebuildCache(RebuildCache),
}
