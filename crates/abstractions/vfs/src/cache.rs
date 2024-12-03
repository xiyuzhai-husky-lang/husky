use std::sync::RwLock;

use crate::*;
use vec_like::VecSet;

pub struct VfsCache {
    files: DashMap<PathBuf, File>,
    current_dir: PathBuf,
    // watcher: Option<VfsWatcher>,
}

impl Default for VfsCache {
    fn default() -> Self {
        Self {
            files: Default::default(),
            current_dir: match std::env::current_dir() {
                Ok(dir) => std::path::absolute(dir).expect("valid path"),
                Err(_e) => todo!(),
            },
            // watcher: None,
        }
    }
}

impl VfsCache {
    pub(crate) fn files(&self) -> &DashMap<PathBuf, File> {
        &self.files
    }

    pub fn current_dir(&self) -> &Path {
        &self.current_dir
    }
}
