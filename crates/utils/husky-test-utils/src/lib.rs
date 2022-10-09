#[cfg(feature = "expect")]
pub mod expect;

use std::path::{Path, PathBuf};

use husky_path_utils::collect_package_dirs;

#[derive(Debug)]
pub enum TestResult {
    Success,
    Failure,
}

pub fn test_all_packages_in_dir(dir: &Path, f: impl Fn(&Path) -> TestResult) {
    if !dir.is_dir() {
        panic!("{:?} is not a directory", dir)
    }
    let package_paths = collect_package_dirs(dir);
    println!(
        "\n{}Running{} tests on {} example packages:",
        husky_print_utils::CYAN,
        husky_print_utils::RESET,
        package_paths.len()
    );

    let mut packages_failed: Vec<PathBuf> = vec![];

    for package_path in package_paths {
        println!(
            "\n{}test{} {}",
            husky_print_utils::CYAN,
            husky_print_utils::RESET,
            package_path.as_os_str().to_str().unwrap(),
        );
        match f(&package_path) {
            TestResult::Success => (),
            TestResult::Failure => packages_failed.push(package_path),
        }
    }
}
