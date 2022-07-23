use husky_compile_test::*;
use husky_compile_time::*;
use husky_test_utils::TestResult;
use std::path::Path;

pub(super) fn test_diagnostics(package_dir: &Path) -> TestResult {
    test_all_source_files(
        package_dir,
        "diagnostics.txt",
        |compile_time, file| match compile_time.module(file) {
            Ok(module) => compile_time.diagnostics_reserve(module).data().clone(),
            Err(e) => vec![e.into()],
        },
    )
}
