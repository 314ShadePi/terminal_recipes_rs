pub trait Command: Clone {
    type Err;
    const CMD: &'static str;

    fn run(&self) -> Result<(), Self::Err>;
    fn new_cmd(params: Vec<String>) -> Result<Self, Self::Err>;
}
