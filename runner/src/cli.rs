use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    // TODO: Spec out a config file. If we need one.
    // /// Sets a custom config file
    // #[arg(short, long, value_name = "FILE")]
    // config: Option<PathBuf>,
    /// Generate more verbose output
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    #[arg(short, long)]
    pub day: Option<u8>,

    #[arg(short, long)]
    pub year: Option<u16>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Log in to the Advent of Code website for input downloading
    Login,

    /// Download problem inputs from Advent of Code
    Input,

    /// Do setup work for a given day or year
    Prep,

    /// Run a specific day's solution
    Run,

    /// Benchmark your solution code with more precision
    Bench,
    // /// Generate flamegraphs of CPU time used by your solution code
    // Flamegraph,

    // /// Run the Coz causal profiler on your solution code
    // Profile,
}
