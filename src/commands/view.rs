use crate::recipe::Recipe;
use cmd_sys::Command;

#[derive(Clone)]
pub struct View {
    recipe: String,
}

impl Command for View {
    const CMD: &'static str = "view";

    fn run(&self) -> anyhow::Result<()> {
        let recipe = Recipe::get_recipe(&self.recipe, true)?;

        println!("{recipe}");
        Ok(())
    }

    fn new_cmd(params: Vec<String>) -> anyhow::Result<Self> {
        Ok(Self {
            recipe: params.join(" "),
        })
    }
}
