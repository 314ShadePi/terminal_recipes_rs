use crate::cache::Cache;
use anyhow::{anyhow, bail};
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;
use std::fs::read_to_string;

#[derive(Deserialize, Serialize, Clone)]
pub struct Recipe {
    pub name: String,
    pub ingredients: Vec<Ingredient>,
    pub steps: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Ingredient {
    pub amount: String,
    pub name: String,
}

impl Recipe {
    #[tracing::instrument(name = "Recipe::get_recipe()")]
    pub fn get_recipe(recipe: &str, first: bool) -> anyhow::Result<Self> {
        let cache = Cache::get_cache(true)?;

        let recipe = match cache.entries.iter().find(|entry| entry.name == recipe) {
            None => {
                if first {
                    Cache::rebuild()?;
                    match Self::get_recipe(recipe, false) {
                        Ok(r) => r,
                        Err(e) => {
                            return Err(e);
                        }
                    }
                } else {
                    tracing::error!("Recipe doesn't exist.");
                    bail!("Recipe doesn't exist.")
                }
            }
            Some(recipe) => {
                let recipe = match read_to_string(recipe.path.clone()) {
                    Ok(s) => s,
                    Err(e) => {
                        let e = anyhow!(e);
                        let e = e.context("Couldn't read recipe.");
                        tracing::error!("{:?}", e);
                        return Err(e);
                    }
                };

                match serde_json::from_str::<Self>(&recipe) {
                    Ok(r) => r,
                    Err(e) => {
                        let e = anyhow!(e);
                        let e = e.context("Couldn't deserialize recipe.");
                        tracing::error!("{:?}", e);
                        return Err(e);
                    }
                }
            }
        };

        Ok(recipe)
    }
}

impl std::fmt::Display for Recipe {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "# {}", self.name)?;
        writeln!(f, "Ingredients:")?;
        for (idx, e) in self.ingredients.iter().enumerate() {
            writeln!(f, "\t{idx}. {e}")?;
        }
        match &self.steps {
            None => {
                write!(f, "No steps to follow. Bon appetit!")?;
            }
            Some(steps) => {
                writeln!(f, "Steps:")?;
                for (idx, e) in steps.iter().enumerate() {
                    writeln!(f, "\t{idx}. {e}")?;
                }
                write!(f, "Bon appetit!")?;
            }
        }
        Ok(())
    }
}

impl std::fmt::Display for Ingredient {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.amount, self.name)
    }
}
