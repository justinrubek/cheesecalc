#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
/// Calculate the amount of sodium citrate, liquid, and pasta used to
/// make mac & cheese from a given amount of ingredients on hand
pub(crate) struct Args {
    /// Perform calculations on a given amount of cheese
    #[clap(short, long)]
    pub(crate) cheese: bool,
    /// Perform calculations on a given amount of pasta
    #[clap(short, long)]
    pub(crate) pasta: bool,
    /// The total mass of the ingredient to be used
    pub(crate) mass: f64,
}
