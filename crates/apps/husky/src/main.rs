use std::path::PathBuf;

use clap::{Parser, Subcommand};

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
}

fn main() {
    let _cli = HuskyCli::parse();
    todo!()
}
