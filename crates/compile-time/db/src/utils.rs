use std::path::Path;

use test_utils::{compare_saved_data, TestResult};

use crate::*;

impl HuskyLangCompileTime {
    fn map_all_modules<T>(
        &self,
        f: impl Fn(&HuskyLangCompileTime, EntityRoutePtr) -> T,
    ) -> HashMap<EntityRoutePtr, T> {
        self.all_modules()
            .into_iter()
            .map(|module| (module, f(self, module)))
            .collect()
    }
}

pub fn test_all_modules<T>(
    package_dir: &Path,
    filename: &str,
    f: impl Fn(&HuskyLangCompileTime, EntityRoutePtr) -> T,
) -> TestResult
where
    T: std::fmt::Debug + PartialEq,
{
    let mut compile_time = HuskyLangCompileTime::default();
    compile_time.load_package(package_dir);
    let diagnostics_table = compile_time.map_all_modules(f);
    compare_saved_data(&diagnostics_table, &package_dir.join(filename))
}
