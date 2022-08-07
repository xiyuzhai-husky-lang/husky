use clap::{Parser, Subcommand};
use husky_debugger::*;
use husky_print_utils::p;
use husky_root_static_defn::{__Linkage, __StaticLinkageKey};
use libloading::{Library, Symbol};
use std::path::PathBuf;

#[derive(Parser)]
#[clap(name = "husky-debugger")]
#[clap(author = "Xiyu Zhai <dirac12345@gmail.com>")]
pub struct HuskyDebuggerCli {
    #[clap(short, long, value_parser)]
    verbose: bool,
    #[clap(subcommand)]
    command: HuskyDebuggerCommands,
}

#[derive(Subcommand)]
enum HuskyDebuggerCommands {
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
    let cli = HuskyDebuggerCli::parse();
    match cli.command {
        HuskyDebuggerCommands::Run { package_dir } => {
            debugger_run(package_dir, cli.verbose).await.unwrap()
        }
        HuskyDebuggerCommands::Test { packages_dir } => {
            debugger_test(packages_dir, cli.verbose).await
        }
    }
}
