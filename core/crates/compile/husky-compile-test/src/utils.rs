use husky_entity_syntax::EntitySyntaxQueryGroup;

use crate::*;

pub fn test_all_source_files<T>(
    package_dir: &Path,
    extension: &str,
    f: impl Fn(&HuskyComptime, FilePtr) -> T,
) -> TestResult
where
    T: HuskyDisplay,
{
    let mut comptime = HuskyComptime::new_default(__resolve_root_defn);
    comptime.load_package(package_dir);
    for file in comptime.all_source_files() {
        match compare_saved_data(&f(&comptime, file), &file.with_extension(extension)) {
            TestResult::Success => (),
            TestResult::Failure => return TestResult::Failure,
        }
    }
    TestResult::Success
}

pub fn print_all_source_files_analysis(
    package_dir: &Path,
    title: &str,
    f: impl Fn(&HuskyComptime, FilePtr) -> String,
) {
    let mut comptime = HuskyComptime::new_default(__resolve_root_defn);
    comptime.load_package(package_dir);
    for file in comptime.all_source_files() {
        println!("{} for file: {:?}:\n", title, file);
        println!("{}", &f(&comptime, file))
    }
}
