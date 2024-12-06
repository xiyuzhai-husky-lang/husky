use crate::*;
use std::sync::RwLock;
use vec_like::VecSet;

pub struct BaseVfsCache {
    files: DashMap<PathBuf, File>,
    current_dir: PathBuf,
}

impl Default for BaseVfsCache {
    fn default() -> Self {
        Self {
            files: Default::default(),
            current_dir: match std::env::current_dir() {
                Ok(dir) => std::path::absolute(dir).expect("valid path"),
                Err(_e) => todo!(),
            },
        }
    }
}

impl BaseVfsCache {
    pub(crate) fn files(&self) -> &DashMap<PathBuf, File> {
        &self.files
    }

    pub fn current_dir(&self) -> &Path {
        &self.current_dir
    }
}
