// Only used for config files (at the moment)
//use std::path::PathBuf;

use clap::{Parser, Subcommand};

use aoc_zen_runner::commands as cmds;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    // TODO: Spec out a config file. If we need one.
    // /// Sets a custom config file
    // #[arg(short, long, value_name = "FILE")]
    // config: Option<PathBuf>,

    /// Generate more verbose output
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
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

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Login) => cmds::login(),
        Some(Commands::Input) => cmds::input(),
        Some(Commands::Prep) => cmds::prepare(),
        Some(Commands::Run) => cmds::run(),
        Some(Commands::Bench) => cmds::benchmark(),
        None => cmds::run(),
    }
}
