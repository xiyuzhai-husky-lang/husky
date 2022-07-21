use std::path::{Path, PathBuf};

use path_utils::collect_all_package_dirs;
use print_utils::p;
mod flags;
mod mode;
mod print_diagnostics;
mod print_folding_ranges;
mod print_qualified_tys;
mod print_semantic_tokens;

use mode::*;
use print_diagnostics::*;
use print_folding_ranges::*;
use print_qualified_tys::*;
use print_semantic_tokens::*;

pub fn print_all() {
    let flags = match flags::HuskyAnalyzerPrinter::from_env() {
        Ok(flags) => flags,
        Err(e) => {
            p!(e);
            panic!()
        }
    };
    let mode = AnalyzerPrinterMode::from_flags(&flags);
    let dir = flags.dir;
    print_all_packages_in_dir(
        &dir,
        match mode {
            AnalyzerPrinterMode::PrintDiagnostics => print_diagnostics,
            AnalyzerPrinterMode::PrintFoldingRanges => print_folding_ranges,
            AnalyzerPrinterMode::PrintSemanticTokens => print_semantic_tokens,
            AnalyzerPrinterMode::PrintQualifiedTys => print_qualified_tys,
        },
    )
}

pub fn print_all_packages_in_dir(dir: &Path, f: impl Fn(&Path)) {
    if !dir.is_dir() {
        panic!("{:?} is not a directory", dir)
    }
    let package_paths = collect_all_package_dirs(dir);
    println!(
        "\n{}Printing{} analysis on {} packages:",
        print_utils::CYAN,
        print_utils::RESET,
        package_paths.len()
    );

    let mut packages_failed: Vec<PathBuf> = vec![];

    for package_path in package_paths {
        println!(
            "\n{}analyze{} {}:\n",
            print_utils::CYAN,
            print_utils::RESET,
            package_path.as_os_str().to_str().unwrap(),
        );
        f(&package_path)
    }
}
