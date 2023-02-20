use crate::cache::Cache;
use cmd_sys::Command;

#[derive(Clone)]
pub struct RebuildCache;

impl Command for RebuildCache {
    const CMD: &'static str = "rebuild-cache";

    fn run(&self) -> anyhow::Result<()> {
        println!("Rebuilding cache, please wait...");
        Cache::rebuild()?;
        println!("Cache rebuilt!");
        Ok(())
    }

    fn new_cmd(_params: Vec<String>) -> anyhow::Result<Self> {
        Ok(Self {})
    }
}
