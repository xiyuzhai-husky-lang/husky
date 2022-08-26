use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// src
    #[clap(value_parser, name = "src")]
    src: PathBuf,
}

fn main() {
    let cli = Cli::parse();
    println!("Hello, world!");
}
