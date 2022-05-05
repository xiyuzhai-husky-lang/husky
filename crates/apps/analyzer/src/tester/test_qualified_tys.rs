use compile_time_db::{utils::test_all_modules, *};
use print_utils::p;
use std::path::Path;
use test_utils::TestResult;
use token::AbsSemanticToken;

pub(super) fn test_qualified_tys(package_dir: &Path) -> TestResult {
    test_all_modules(package_dir, "qualified_tys.txt", |compile_time, module| {
        compile_time
            .qualified_ty_sheet(compile_time.module_file(module).unwrap())
            .unwrap()
    })
}
