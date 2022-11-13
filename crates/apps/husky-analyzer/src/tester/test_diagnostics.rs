
use husky_test_utils::TestResult;
use std::path::Path;

pub(super) fn test_diagnostics(_package_dir: &Path) -> TestResult {
    todo!()
    // test_all_source_files(
    //     package_dir,
    //     "diagnostics.txt",
    //     |comptime, file| match comptime.module(file) {
    //         Ok(module) => comptime.diagnostics_reserve(module).data().clone(),
    //         Err(e) => vec![e.into()],
    //     },
    // )
}
