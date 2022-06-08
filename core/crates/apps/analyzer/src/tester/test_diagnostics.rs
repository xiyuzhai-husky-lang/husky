use compile_time_db::{utils::test_all_source_files, *};
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
