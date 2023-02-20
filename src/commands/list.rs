use crate::cache::Cache;
use cmd_sys::Command;

#[derive(Debug, Clone)]
pub struct List;

impl Command for List {
    const CMD: &'static str = "list";

    #[tracing::instrument]
    fn run(&self) -> anyhow::Result<()> {
        let cache = Cache::get_cache(true)?;

        print!("{cache}");

        Ok(())
    }

    #[tracing::instrument]
    fn new_cmd(_params: Vec<String>) -> anyhow::Result<Self> {
        Ok(Self {})
    }
}
