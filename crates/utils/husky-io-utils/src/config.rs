use std::path::Path;

use crate::{relative_path_pattern::RelativePathPattern, *};
use composite_pattern::CompositePattern;

pub struct FileVisitConfig {
    pub regular_file_filter: CompositePattern<RelativePathPattern>,
    pub dir_filter: CompositePattern<RelativePathPattern>,
    pub verbose: bool,
}

pub struct FileSyncConfig {
    pub src_root: RelativePathBuf,
    pub dst_root: RelativePathBuf,
    pub file_visit: FileVisitConfig,
}

impl FileVisitConfig {
    fn filter(&self, root: &RelativePath, path: &Path) -> bool {
        assert!(path.exists());
        let rel_path = root.relative(RelativePathBuf::from_path(path).unwrap());
        if path.is_file() {
            self.regular_file_filter.contains(&rel_path)
        } else {
            assert!(path.is_dir());
            self.dir_filter.contains(&rel_path)
        }
    }
}

impl FileSyncConfig {
    pub fn filter_src(&self, path: &Path) -> bool {
        self.file_visit.filter(&self.src_root, path)
    }

    pub fn filter_dst(&self, path: &Path) -> bool {
        self.file_visit.filter(&self.dst_root, path)
    }

    pub fn verbose(&self) -> bool {
        self.file_visit.verbose
    }
}
