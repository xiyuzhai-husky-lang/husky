mod compare;

pub use compare::*;

use std::path::{Path, PathBuf};

use path_utils::collect_all_package_dirs;

#[derive(Debug)]
pub enum TestResult {
    Success,
    Failed,
}

pub fn test_all_packages_in_dir(dir: PathBuf, f: impl Fn(&Path) -> TestResult) {
    if !dir.is_dir() {
        panic!("{:?} is not a directory", &dir)
    }
    let pack_paths = collect_all_package_dirs(dir);
    println!(
        "\n{}Running{} tests on {} example packages:",
        print_utils::CYAN,
        print_utils::RESET,
        pack_paths.len()
    );

    let mut packages_failed: Vec<PathBuf> = vec![];

    for package_path in pack_paths {
        println!(
            "\n{}test{} {}",
            print_utils::CYAN,
            print_utils::RESET,
            package_path.as_os_str().to_str().unwrap(),
        );
        match f(&package_path) {
            TestResult::Success => (),
            TestResult::Failed => packages_failed.push(package_path),
        }
    }
}
