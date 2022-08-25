use std::path::Path;

use crate::path_pattern::PathPattern;
use composite_pattern::{AtomicPattern as _, CompositePattern};

pub struct FileVisitConfig {
    pub regular_file_filter: CompositePattern<PathPattern>,
    pub dir_filter: CompositePattern<PathPattern>,
}

impl FileVisitConfig {
    pub fn filter(&self, path: &Path) -> bool {
        assert!(path.exists());
        if path.is_file() {
            self.regular_file_filter.contains(path)
        } else {
            assert!(path.is_dir());
            self.dir_filter.contains(path)
        }
    }
}
