use anyhow::{anyhow, Result};
use cheesecalc::{from_cheese_mass, from_pasta_mass, Ingredient};
use clap::Parser;

mod commands;

fn print_results(items: &[Ingredient]) {
    items.iter().for_each(|ingredient| {
        println!(
            "{}: {}{}",
            ingredient.name, ingredient.quantity, ingredient.unit
        );
    });
}

fn main() -> Result<()> {
    let args = commands::Args::parse();

    match (args.cheese, args.pasta) {
        (false, false) => Err(anyhow!("Must specify either cheese or pasta")),
        (true, true) => Err(anyhow!("Must specify cheese OR pasta, not both")),
        (true, false) => {
            print_results(&from_cheese_mass(args.mass));
            Ok(())
        }
        (false, true) => {
            print_results(&from_pasta_mass(args.mass));
            Ok(())
        }
    }
}
