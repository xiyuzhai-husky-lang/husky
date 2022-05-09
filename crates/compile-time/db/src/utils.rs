use std::path::Path;

use test_utils::{compare_saved_data, TestDisplay, TestResult};

use crate::*;

// impl HuskyLangCompileTime {
//     fn map_all_modules<T>(
//         &self,
//         f: impl Fn(&HuskyLangCompileTime, EntityRoutePtr) -> T,
//     ) -> HashMap<EntityRoutePtr, T> {
//         self.all_modules()
//             .into_iter()
//             .map(|module| (module, f(self, module)))
//             .collect()
//     }
// }

pub fn test_all_modules<T>(
    package_dir: &Path,
    extension: &str,
    f: impl Fn(&HuskyLangCompileTime, EntityRoutePtr) -> T,
) -> TestResult
where
    T: TestDisplay,
{
    let mut compile_time = HuskyLangCompileTime::default();
    compile_time.load_package(package_dir);
    for module in compile_time.all_modules() {
        let file = compile_time.module_file(module).unwrap();
        match compare_saved_data(&f(&compile_time, module), &file.with_extension(extension)) {
            TestResult::Success => (),
            TestResult::Failed => return TestResult::Failed,
        }
    }
    TestResult::Success
}

pub fn print_all_modules(
    package_dir: &Path,
    title: &str,
    f: impl Fn(&HuskyLangCompileTime, EntityRoutePtr) -> String,
) {
    let mut compile_time = HuskyLangCompileTime::default();
    compile_time.load_package(package_dir);
    for module in compile_time.all_modules() {
        let file = compile_time.module_file(module).unwrap();
        println!("{} for file: {:?}:\n", title, file);
        println!("{}", &f(&compile_time, module))
    }
}
