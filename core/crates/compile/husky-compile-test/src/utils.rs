use entity_syntax::EntitySyntaxQueryGroup;

use crate::*;

pub fn test_all_source_files<T>(
    package_dir: &Path,
    extension: &str,
    f: impl Fn(&HuskyCompileTime, FilePtr) -> T,
) -> TestResult
where
    T: TestDisplay,
{
    let mut compile_time = HuskyCompileTime::new(static_root_defn);
    compile_time.load_package(package_dir);
    for file in compile_time.all_source_files() {
        match compare_saved_data(&f(&compile_time, file), &file.with_extension(extension)) {
            TestResult::Success => (),
            TestResult::Failed => return TestResult::Failed,
        }
    }
    TestResult::Success
}

pub fn print_all_source_files_analysis(
    package_dir: &Path,
    title: &str,
    f: impl Fn(&HuskyCompileTime, FilePtr) -> String,
) {
    let mut compile_time = HuskyCompileTime::new(static_root_defn);
    compile_time.load_package(package_dir);
    for file in compile_time.all_source_files() {
        println!("{} for file: {:?}:\n", title, file);
        println!("{}", &f(&compile_time, file))
    }
}
