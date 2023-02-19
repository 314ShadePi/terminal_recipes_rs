use crate::cache::Cache;
use cmd_sys::Command;

#[derive(Clone)]
pub struct RebuildCache;

impl Command for RebuildCache {
    type Err = ();
    const CMD: &'static str = "rebuild-cache";

    fn run(&self) -> Result<(), Self::Err> {
        println!("Rebuilding cache, please wait...");
        Cache::rebuild()?;
        println!("Cache rebuilt!");
        Ok(())
    }

    fn new_cmd(_params: Vec<String>) -> Result<Self, Self::Err> {
        Ok(Self {})
    }
}
