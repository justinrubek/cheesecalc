use anyhow::{anyhow, Result};
use clap::Parser;

mod commands;

struct Ingredient {
    name: String,
    unit: String,
    quantity: f64,
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

fn print_results(items: &[Ingredient]) {
    items.iter().for_each(|ingredient| {
        println!(
            "{}: {}{}",
            ingredient.name, ingredient.quantity, ingredient.unit
        );
    });
}

fn from_cheese(cheese_mass: f64) -> Vec<Ingredient> {
    vec![
        Ingredient::new("cheese", "g", cheese_mass),
        Ingredient::new("liquid", "ml", cheese_mass * 0.93),
        Ingredient::new("pasta", "g", cheese_mass * 0.84),
        Ingredient::new("sodium citrate", "g", cheese_mass * 0.04),
    ]
}

fn from_pasta(pasta_mass: f64) -> Vec<Ingredient> {
    let cheese_mass = pasta_mass / 0.84;

    vec![
        Ingredient::new("cheese", "g", cheese_mass),
        Ingredient::new("liquid", "ml", cheese_mass * 0.93),
        Ingredient::new("pasta", "g", pasta_mass),
        Ingredient::new("sodium citrate", "g", cheese_mass * 0.04),
    ]
}

fn main() -> Result<()> {
    let args = commands::Args::parse();

    match (args.cheese, args.pasta) {
        (false, false) => Err(anyhow!("Must specify either cheese or pasta")),
        (true, true) => Err(anyhow!("Must specify cheese OR pasta, not both")),
        (true, false) => {
            print_results(&from_cheese(args.mass));
            Ok(())
        }
        (false, true) => {
            print_results(&from_pasta(args.mass));
            Ok(())
        }
    }
}
