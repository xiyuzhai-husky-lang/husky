use husky_compile_test::*;
use husky_compile_time::*;
use std::path::Path;
use test_utils::TestResult;

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
