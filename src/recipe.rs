use serde::{Deserialize, Serialize};

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
