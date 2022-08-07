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
        package_dirs: PathBuf,
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
        HuskyDebuggerCommands::Launch { package_dir } => todo!(),
        HuskyDebuggerCommands::Test { package_dirs } => todo!(),
    }
    todo!();
    // let flags = HuskyDebuggerFlags::from_env().expect("invalid arguments");
    // let opt_library: Option<Library> = if let Some(cdylib) = flags.cdylib {
    //     Some(unsafe { Library::new(cdylib) }.expect("it should work"))
    // } else {
    //     None
    // };
    // let linkages_from_cdylib: &'static [(__StaticLinkageKey, __Linkage)] = opt_library
    //     .as_ref()
    //     .map(|library| unsafe {
    //         library
    //             .get::<GetLinkagesFromCDylib>(b"get_linkages")
    //             .expect("what")()
    //     })
    //     .unwrap_or(&[]);
    // let mode: Mode = flags.mode.into();
    // let package_dir: PathBuf = flags.package_dir.unwrap().into();
    // mode.apply(&package_dir, linkages_from_cdylib).await
}
