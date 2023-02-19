use crate::cache::Cache;
use cmd_sys::Command;

#[derive(Clone)]
pub struct List;

impl Command for List {
    type Err = ();
    const CMD: &'static str = "list";

    fn run(&self) -> Result<(), Self::Err> {
        let cache = Cache::get_cache(true)?;

        print!("{}", cache);

        Ok(())
    }

    fn new_cmd(_params: Vec<String>) -> Result<Self, Self::Err> {
        Ok(Self {})
    }
}
