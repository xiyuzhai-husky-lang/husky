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
    Launch {
        #[clap(value_parser)]
        package_dir: PathBuf,
    },
    /// serve traces on first package with error
    Test {
        #[clap(value_parser)]
        packages_dir: PathBuf,
    },
}
// use std::path::PathBuf;

// xflags::xflags! {
//     cmd husky-debugger-flags
//     {
//         optional --package-dir package_dir: PathBuf
//         optional --warn-missing-linkage
//         optional -v, --verbose
//         optional --sample-id sample_id: String
//         optional --mode mode: String
//         optional --cdylib cdylib: String
//         optional -c, --compiled
//     }
// }

#[tokio::main]
async fn main() {
    let cli = HuskyDebuggerCli::parse();
    match cli.command {
        HuskyDebuggerCommands::Launch { package_dir } => debugger_launch(package_dir, cli.verbose),
        HuskyDebuggerCommands::Test { packages_dir } => debugger_test(packages_dir, cli.verbose),
    }
}
