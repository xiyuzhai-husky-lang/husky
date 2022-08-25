use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// package dir
    #[clap(value_parser)]
    package_dir: PathBuf,
    /// test class
    #[clap(subcommand)]
    test_class: TestAddCommand,
}

#[derive(Subcommand)]
enum TestAddCommand {
    /// add test for anything about development, folding ranges, inference, etc.
    ToDevtimeTests,
    /// add test for anything about compilation, Rust/Zig compiler errors, etc.
    ToComptimeTests,
    /// add test for anything about runtime
    ToRuntimeTests,
}

fn main() {
    let cli = Cli::parse();
}
