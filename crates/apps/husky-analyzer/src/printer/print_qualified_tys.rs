use crate::*;
use husky_comptime::*;
use husky_display_utils::{HuskyDisplay, HuskyDisplayConfig};
use std::path::Path;
pub fn print_qualified_tys(package_dir: &Path) {
    print_all_source_files_analysis(package_dir, "qualified tys", |comptime, file| {
        comptime
            .qualified_ty_sheet(file)
            .unwrap()
            .print_inherent(HuskyDisplayConfig {
                colored: true,
                indent: 4,
            })
    })
}
