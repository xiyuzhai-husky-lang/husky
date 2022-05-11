use std::path::Path;

use compile_time_db::{utils::print_all_source_files_analysis, *};
use test_utils::{TestDisplayConfig, TestDisplay};

pub fn print_qualified_tys(package_dir: &Path) {
    print_all_source_files_analysis(package_dir, "qualified tys", |compile_time, file| {
        compile_time
            .qualified_ty_sheet(file)
            .unwrap()
            .print_inherent(TestDisplayConfig {
                colored: true,
                indent: 4,
            })
    })
}
