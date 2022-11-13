use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// src
    #[clap(value_parser, name = "src")]
    src: PathBuf,
}

fn main() {
    let _cli = Cli::parse();
    println!("Hello, world!");
}
