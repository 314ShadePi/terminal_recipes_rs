use crate::cache::Cache;
use cmd_sys::Command;

#[derive(Debug, Clone)]
pub struct RebuildCache;

impl Command for RebuildCache {
    const CMD: &'static str = "rebuild-cache";

    #[tracing::instrument(name = "RebuildCache::run()")]
    fn run(&self) -> anyhow::Result<()> {
        println!("Rebuilding cache, please wait...");
        Cache::rebuild()?;
        println!("Cache rebuilt!");
        Ok(())
    }

    #[tracing::instrument(name = "RebuildCache::new_cmd()")]
    fn new_cmd(_params: Vec<String>) -> anyhow::Result<Self> {
        Ok(Self {})
    }
}
