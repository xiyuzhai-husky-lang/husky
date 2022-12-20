mod demo;

use clap::{Parser, Subcommand};
use demo::*;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(name = "husky-developer")]
#[clap(author = "Xiyu Zhai <dirac12345@gmail.com>")]
pub struct HuskyCli {
    #[clap(short, long, value_parser)]
    verbose: bool,
    #[clap(subcommand)]
    command: HuskyCommands,
}

#[derive(Subcommand)]
enum HuskyCommands {
    /// compile and serve traces on given package
    Debug {
        #[clap(value_parser)]
        package_dir: PathBuf,
    },
    /// serve traces on first package with error
    Test {
        #[clap(value_parser)]
        packages_dir: PathBuf,
    },
    /// format
    Fmt {
        #[clap(value_parser)]
        packages_dir: PathBuf,
    },
    /// demonstrate examples
    Demo {
        #[clap(subcommand)]
        target: HuskyDemoTarget,
    },
}

fn main() {
    let cli = HuskyCli::parse();
    match cli.command {
        HuskyCommands::Debug { package_dir: _ } => todo!(),
        HuskyCommands::Test { packages_dir: _ } => todo!(),
        HuskyCommands::Fmt { packages_dir: _ } => todo!(),
        HuskyCommands::Demo { target } => demo(target),
    }
}
