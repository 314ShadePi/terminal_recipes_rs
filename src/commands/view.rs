use crate::recipe::Recipe;
use cmd_sys::Command;

#[derive(Clone)]
pub struct View {
    recipe: String,
}

impl Command for View {
    type Err = ();
    const CMD: &'static str = "view";

    fn run(&self) -> Result<(), Self::Err> {
        let Ok(recipe) = Recipe::get_recipe(&self.recipe, true) else {
            return Err(());
        };

        println!("{recipe}");
        Ok(())
    }

    fn new_cmd(params: Vec<String>) -> Result<Self, Self::Err> {
        Ok(Self {
            recipe: params.join(" "),
        })
    }
}
