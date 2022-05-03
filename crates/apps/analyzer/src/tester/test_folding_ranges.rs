use compile_time_db::{utils::*, *};
use std::path::Path;
use test_utils::*;

pub(super) fn test_folding_ranges(package_dir: &Path) -> TestResult {
    test_all_modules(package_dir, "folding_ranges.txt", |compile_time, module| {
        compile_time
            .tokenized_text(compile_time.module_file(module).unwrap())
            .unwrap()
            .folding_ranges()
    })
}
