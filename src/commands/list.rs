use crate::cache::Cache;
use cmd_sys::Command;

#[derive(Debug, Clone)]
pub struct List;

impl Command for List {
    const CMD: &'static str = "list";
    const HELP: &'static str = "";
    const HELP_LONG: &'static str = "";

    #[tracing::instrument(name = "List::run()")]
    fn run(&self) -> anyhow::Result<()> {
        let cache = Cache::get_cache(true)?;

        print!("{cache}");

        Ok(())
    }

    #[tracing::instrument(name = "List::new_cmd()")]
    fn new_cmd(_params: Vec<String>) -> anyhow::Result<Self> {
        Ok(Self {})
    }
}
