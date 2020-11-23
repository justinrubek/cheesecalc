use clap::Clap;

/// Calculate the amount of sodium citrate, liquid, and pasta used to
/// make mac & cheese from a given amount of cheese on hand
#[derive(Clap)]
#[clap(version = "0.1.0", author = "Justin Rubek")]
struct Opts {
    /// The total mass of cheese to be used, in grams
    cheese_mass: f64,
}

fn main() {
    let opts: Opts = Opts::parse();

    println!("Cheese: {}g", opts.cheese_mass);
    println!("Liquid: {}ml", opts.cheese_mass * 0.93);
    println!("Sodium citrate: {}g", opts.cheese_mass * 0.04);
    println!("Pasta {}g", opts.cheese_mass * 0.84);
}
