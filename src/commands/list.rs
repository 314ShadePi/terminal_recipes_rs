use crate::cache::Cache;
use crate::CACHE_FILE;
use cmd_sys::Command;
use std::fs::read_to_string;

#[derive(Clone)]
pub struct List;

impl Command for List {
    type Err = ();
    const CMD: &'static str = "list";

    fn run(&self) -> Result<(), Self::Err> {
        let cache = match read_to_string(&CACHE_FILE.clone()) {
            Ok(f) => f,
            Err(e) => {
                println!("ERROR: {}", e);
                return Err(());
            }
        };

        let cache = match serde_json::from_str::<Cache>(&cache) {
            Ok(c) => c,
            Err(e) => {
                println!("ERROR: {}", e);
                return Err(());
            }
        };

        print!("{}", cache);

        Ok(())
    }

    fn new_cmd(_params: Vec<String>) -> Result<Self, Self::Err> {
        Ok(Self {})
    }
}
