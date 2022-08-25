mod diagnostics;
mod folding_ranges;
mod semantic_tokens;

use diagnostics::*;
use folding_ranges::*;
use semantic_tokens::*;
use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// src
    #[clap(value_parser, name = "src")]
    src: PathBuf,
    #[clap(value_parser, name = "dst husky dev root")]
    dst_husky_dev_root: PathBuf,
    /// test class
    #[clap(subcommand, name = "class")]
    test_class: TestClass,
}

#[derive(Subcommand)]
enum TestClass {
    FoldingRanges {
        #[clap(subcommand)]
        subclass: FoldingRangesTestOrder,
    },
    SemanticTokens {
        #[clap(subcommand)]
        subclass: SemanticTokensTestOrder,
    },
    Diagnostics {
        #[clap(subcommand)]
        subclass: DiagnosticsTestOrder,
    },
    QualifiedTys,
    Comptime,
    Runtime,
}

impl TestClass {
    fn relative_path_str(&self) -> &'static str {
        match self {
            TestClass::FoldingRanges { subclass } => subclass.relative_path_str(),
            TestClass::SemanticTokens { subclass } => subclass.relative_path_str(),
            TestClass::Diagnostics { subclass } => subclass.relative_path_str(),
            TestClass::QualifiedTys => todo!(),
            TestClass::Comptime => todo!(),
            TestClass::Runtime => todo!(),
        }
    }
}

fn main() {
    let cli = Cli::parse();
    let src = cli.src;
    let dst_husky_dev_root = cli.dst_husky_dev_root;
    let test_dir = dst_husky_dev_root.join(cli.test_class.relative_path_str());
}
