use compile_time_db::{utils::test_all_modules, *};
use std::path::Path;
use test_utils::TestResult;

pub(super) fn test_diagnostics(package_dir: &Path) -> TestResult {
    test_all_modules(package_dir, "diagnostics.txt", |compile_time, module| {
        compile_time.diagnostics_reserve(module).data().clone()
    })
}
