mod test_class;

use clap::{Parser, Subcommand};
use husky_check_utils::should;
use husky_cli_utils::ask::ask_user_for_permission;
use husky_io_utils::{
    file_sync::diff_file_sync, relative_path_pattern::RelativePathPattern, FileVisitConfig,
};
use husky_print_utils::*;
use std::path::{Path, PathBuf};
use test_class::*;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// src
    #[clap(value_parser, name = "src")]
    src: PathBuf,
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
    QualifiedTys {
        #[clap(subcommand)]
        subclass: QualifiedTysTestOrder,
    },
    Comptime,
    Runtime,
}

impl TestClass {
    fn relative_path_str(&self) -> &'static str {
        match self {
            TestClass::FoldingRanges { subclass } => subclass.relative_path_str(),
            TestClass::SemanticTokens { subclass } => subclass.relative_path_str(),
            TestClass::Diagnostics { subclass } => subclass.relative_path_str(),
            TestClass::QualifiedTys { subclass } => subclass.relative_path_str(),
            TestClass::Comptime => todo!(),
            TestClass::Runtime => todo!(),
        }
    }
}

fn main() {
    let cli = Cli::parse();
    let src_package_dir = cli.src;
    check_is_package(&src_package_dir);
    let src_package_name = src_package_dir.file_name().unwrap().to_str().unwrap();
    let dst_parent_dir: PathBuf = cli.test_class.relative_path_str().into();
    should!(
        dst_parent_dir.exists(),
        "dst_parent_dir = {dst_parent_dir:?}"
    );
    let dst_package_dir = gen_dst_package_dir(&dst_parent_dir, src_package_name);
    attempt_to_save_husky_code(&src_package_dir, &dst_package_dir)
}

fn check_is_package(src: &PathBuf) {
    if !src.join("main.hsy").exists() {
        panic!(
            "{:?}, is not a valid package because `main.hsy` can't be found",
            src
        )
    }
}

fn gen_dst_package_dir(dst_parent_dir: &Path, src_package_name: &str) -> PathBuf {
    let mut index = 0;
    for entry in std::fs::read_dir(dst_parent_dir).unwrap() {
        let entry = entry.unwrap();
        let subpath = entry.path();
        assert!(subpath.is_dir());
        let temp_package_name = subpath.file_name().unwrap().to_str().unwrap();
        let splits: Vec<_> = temp_package_name.split("--").collect();
        if splits[0] == src_package_name {
            let temp_index_str = splits[1];
            let temp_index: u32 = temp_index_str.parse().unwrap();
            if index <= temp_index {
                index = temp_index + 1
            }
        }
    }
    dst_parent_dir.join(format!("{src_package_name}--{index}"))
}

fn attempt_to_save_husky_code(src_package_dir: &Path, dst_package_dir: &Path) {
    let permitted = ask_user_for_permission(format!(
        "Do you want to save {GREEN}{}{RESET} as {CYAN}{}{RESET}",
        src_package_dir.as_os_str().to_str().unwrap(),
        dst_package_dir.as_os_str().to_str().unwrap()
    ));
    if permitted {
        diff_file_sync(
            src_package_dir,
            dst_package_dir,
            FileVisitConfig {
                regular_file_filter: RelativePathPattern::extension_is_among(["hsy", "toml"]),
                dir_filter: RelativePathPattern::ignore_paths([
                    "__rust_gen__",
                    "__rust_gen_cache__",
                ]),
                verbose: false,
            },
        )
    } else {
        println!("{RED}Abort save.{RESET}")
    }
}
