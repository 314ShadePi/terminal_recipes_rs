pub trait Command: Clone {
    const CMD: &'static str;
    const HELP_SHORT: &'static str;
    const HELP_LONG: &'static str;

    fn run(&self) -> anyhow::Result<()>;
    fn new_cmd(params: Vec<String>) -> anyhow::Result<Self>;
}
