use crate::recipe::Recipe;
use cmd_sys::Command;

#[derive(Debug, Clone)]
pub struct View {
    recipe: String,
}

impl Command for View {
    const CMD: &'static str = "view";
    const HELP: &'static str = "";
    const HELP_LONG: &'static str = "";

    #[tracing::instrument(name = "View::run()")]
    fn run(&self) -> anyhow::Result<()> {
        let recipe = Recipe::get_recipe(&self.recipe, true)?;

        println!("{recipe}");
        Ok(())
    }

    #[tracing::instrument(name = "View::new_cmd()")]
    fn new_cmd(params: Vec<String>) -> anyhow::Result<Self> {
        Ok(Self {
            recipe: params.join(" "),
        })
    }
}
