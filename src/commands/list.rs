use cmd_sys::Command;

#[derive(Clone)]
pub struct List;

impl Command for List {
    type Err = ();
    const CMD: &'static str = "list";

    fn run(&self) -> Result<(), Self::Err> {
        println!("List!");
        Ok(())
    }

    fn new_cmd(_params: Vec<String>) -> Result<Self, Self::Err> {
        Ok(Self {})
    }
}
