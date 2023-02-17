use crate::cache::Cache;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;
use std::fs::read_to_string;
use std::path::PathBuf;

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
    pub fn view(cache: PathBuf, recipe_dir: PathBuf, recipe: String) {
        let recipe = match Self::get_recipe(cache.clone(), recipe_dir.clone(), recipe.clone(), true)
        {
            Ok(r) => r,
            Err(_) => {
                println!("Could not get recipe!");
                return;
            }
        };

        println!("{}", recipe);
    }

    fn get_recipe(
        cache: PathBuf,
        recipe_dir: PathBuf,
        recipe: String,
        first_run: bool,
    ) -> Result<Self, ()> {
        let cache_s = match read_to_string(cache.clone()) {
            Ok(s) => s,
            Err(_) => {
                println!("Could not read cache!");
                return Err(());
            }
        };

        let cache_s = match serde_json::from_str::<Cache>(&cache_s) {
            Ok(c) => c,
            Err(_) => return Err(()),
        };

        let recipe = match cache_s.entries.iter().find(|entry| entry.name == recipe) {
            None => {
                if first_run {
                    Cache::rebuild_cache(recipe_dir.clone().join("data/"), recipe_dir.clone());
                    Self::get_recipe(cache.clone(), recipe_dir.clone(), recipe.clone(), false)
                } else {
                    return Err(());
                }
            }
            Some(r) => {
                let recipe_s = match read_to_string(r.path.clone()) {
                    Ok(s) => s,
                    Err(_) => {
                        println!("Could not read recipe!");
                        return Err(());
                    }
                };

                let recipe_s = match serde_json::from_str::<Self>(&recipe_s) {
                    Ok(c) => c,
                    Err(_) => return Err(()),
                };

                Ok(recipe_s)
            }
        };

        recipe
    }
}

impl std::fmt::Display for Recipe {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "# {}", self.name);
        writeln!(f, "Ingredients:");
        for (idx, e) in self.ingredients.iter().enumerate() {
            writeln!(f, "\t{}. {}", idx, e);
        }
        match &self.steps {
            None => {
                write!(f, "No steps to follow. Bon appetit!");
            }
            Some(steps) => {
                writeln!(f, "Steps:");
                for (idx, e) in steps.iter().enumerate() {
                    writeln!(f, "\t{}. {}", idx, e);
                }
                write!(f, "Bon appetit!");
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
