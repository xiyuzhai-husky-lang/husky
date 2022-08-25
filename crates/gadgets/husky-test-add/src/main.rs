use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// src
    #[clap(value_parser, name = "src")]
    src: PathBuf,
    #[clap(value_parser, name = "husky dev root")]
    husky_dev_root: PathBuf,
    /// test class
    #[clap(subcommand, name = "class")]
    to_which_test_class: ToWhichTestClass,
}

#[derive(Subcommand)]
enum ToWhichTestClass {
    DevtimeFoldingRangesTests {
        #[clap(subcommand)]
        subclass: FoldingRangesSubclasses,
    },
    DevtimeSemanticTokensTests,
    DevtimeDiagnosticsTests,
    DevtimeQualifiedTysTests,
    ComptimeTests,
    RuntimeTests,
}

#[derive(Subcommand)]
enum FoldingRangesSubclasses {
    Misc,
}

fn main() {
    let cli = Cli::parse();
    let package_dir = cli.src;
    let to_which_test_class = cli.to_which_test_class;
    match to_which_test_class {
        ToWhichTestClass::DevtimeFoldingRangesTests { .. } => todo!(),
        ToWhichTestClass::DevtimeSemanticTokensTests => todo!(),
        ToWhichTestClass::DevtimeDiagnosticsTests => todo!(),
        ToWhichTestClass::DevtimeQualifiedTysTests => todo!(),
        ToWhichTestClass::ComptimeTests => todo!(),
        ToWhichTestClass::RuntimeTests => todo!(),
    }
}
