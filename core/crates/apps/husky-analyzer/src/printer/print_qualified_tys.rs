use crate::*;
use husky_compile_time::*;
use husky_test_utils::{TestDisplay, TestDisplayConfig};
use std::path::Path;

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
