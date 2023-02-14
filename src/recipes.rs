use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Recipes(pub Vec<Recipe>);

#[derive(Deserialize, Serialize, Clone)]
pub struct Recipe {

}

impl std::ops::Deref for Recipes {
    type Target = Vec<Recipe>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Recipes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}