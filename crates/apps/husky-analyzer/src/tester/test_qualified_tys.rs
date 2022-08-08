use husky_compile_test::test_all_source_files;
use husky_compile_time::*;
use husky_test_utils::TestResult;
use std::path::Path;

pub(super) fn test_qualified_tys(package_dir: &Path) -> TestResult {
    test_all_source_files(package_dir, "qualified_tys.txt", |comptime, file| {
        comptime.qualified_ty_sheet(file).ok()
    })
}
