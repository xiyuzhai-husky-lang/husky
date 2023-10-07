use clap::{Parser, Subcommand};

use std::path::PathBuf;

#[derive(Parser)]
#[clap(name = "husky-developer")]
#[clap(author = "Xiyu Zhai <dirac12345@gmail.com>")]
pub struct HuskyDeveloperCli {
    #[clap(short, long, value_parser)]
    verbose: bool,
    #[clap(subcommand)]
    command: HuskyDeveloperCommands,
}

#[derive(Subcommand)]
enum HuskyDeveloperCommands {
    /// serve traces on given package
    Run {
        #[clap(value_parser)]
        package_dir: PathBuf,
    },
    /// serve traces on first package with error
    Test {
        #[clap(value_parser)]
        packages_dir: PathBuf,
    },
}

#[tokio::main]
async fn main() {
    let _cli = HuskyDeveloperCli::parse();
    todo!()
    // match cli.command {
    //     HuskyDeveloperCommands::Run { package_dir } => dev_run(package_dir).await.unwrap(),
    //     HuskyDeveloperCommands::Test { packages_dir } => dev_test(packages_dir).await,
    // }
}
