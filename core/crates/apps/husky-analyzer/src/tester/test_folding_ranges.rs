use husky_compile_time::{utils::*, *};
use std::path::Path;
use test_utils::*;

pub(super) fn test_folding_ranges(package_dir: &Path) -> TestResult {
    test_all_source_files(package_dir, "folding_ranges.txt", |compile_time, file| {
        compile_time.tokenized_text(file).unwrap().folding_ranges()
    })
}
