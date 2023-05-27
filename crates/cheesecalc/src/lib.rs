use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Ingredient {
    pub name: String,
    pub unit: String,
    pub quantity: f64,
}

impl Ingredient {
    pub fn new(name: &str, unit: &str, quantity: f64) -> Self {
        Self {
            name: name.to_owned(),
            unit: unit.to_owned(),
            quantity,
        }
    }
}

pub fn from_cheese_mass(cheese_mass: f64) -> Vec<Ingredient> {
    vec![
        Ingredient::new("cheese", "g", cheese_mass),
        Ingredient::new("liquid", "ml", cheese_mass * 0.93),
        Ingredient::new("pasta", "g", cheese_mass * 0.84),
        Ingredient::new("sodium citrate", "g", cheese_mass * 0.04),
    ]
}

pub fn from_pasta_mass(pasta_mass: f64) -> Vec<Ingredient> {
    let cheese_mass = (pasta_mass / 0.84).round();

    vec![
        Ingredient::new("cheese", "g", cheese_mass),
        Ingredient::new("liquid", "ml", cheese_mass * 0.93),
        Ingredient::new("pasta", "g", pasta_mass),
        Ingredient::new("sodium citrate", "g", cheese_mass * 0.04),
    ]
}
