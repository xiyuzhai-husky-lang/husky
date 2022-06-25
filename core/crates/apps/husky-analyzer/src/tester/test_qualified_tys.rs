use husky_compile_test::test_all_source_files;
use husky_compile_time::*;
use std::path::Path;
use test_utils::TestResult;

pub(super) fn test_qualified_tys(package_dir: &Path) -> TestResult {
    test_all_source_files(package_dir, "qualified_tys.txt", |compile_time, file| {
        compile_time.qualified_ty_sheet(file).ok()
    })
}
