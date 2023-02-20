pub trait Command: Clone {
    const CMD: &'static str;

    fn run(&self) -> anyhow::Result<()>;
    fn new_cmd(params: Vec<String>) -> anyhow::Result<Self>;
}
