use std::path::Path;

use compile_time_db::{utils::print_all_modules, *};
use test_utils::{TestCompareConfig, TestDisplay};

pub fn print_qualified_tys(package_dir: &Path) {
    print_all_modules(package_dir, "qualified tys", |compile_time, module| {
        compile_time
            .qualified_ty_sheet(compile_time.module_file(module).unwrap())
            .unwrap()
            .print_inherent(TestCompareConfig {
                colored: true,
                indent: 4,
            })
    })
}
