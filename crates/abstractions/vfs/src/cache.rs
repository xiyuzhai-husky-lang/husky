use std::sync::RwLock;

use crate::*;
use husky_fs_specs::{huskyup_install_path, root_corgi_path, FsSpecsError, FsSpecsResult};
use vec_like::VecSet;

pub struct VfsCache {
    files: DashMap<PathBuf, File>,
    corgi_install_path: FsSpecsResult<PathBuf>,
    huskyup_install_path: FsSpecsResult<PathBuf>,
    current_dir: PathBuf,
    // watcher: Option<VfsWatcher>,
}

impl Default for VfsCache {
    fn default() -> Self {
        let corgi_install_path = root_corgi_path();
        assert!(corgi_install_path
            .as_ref()
            .map(|path| path.is_absolute())
            .unwrap_or(true));
        let huskyup_install_path = huskyup_install_path();
        assert!(huskyup_install_path
            .as_ref()
            .map(|path| path.is_absolute())
            .unwrap_or(true));
        Self {
            files: Default::default(),
            corgi_install_path,
            huskyup_install_path,
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

    pub fn corgi_install_path(&self) -> Result<&PathBuf, &FsSpecsError> {
        self.corgi_install_path.as_ref()
    }

    pub fn huskyup_install_path(&self) -> Result<&PathBuf, &FsSpecsError> {
        self.huskyup_install_path.as_ref()
    }

    // pub fn watcher(&self) -> Option<&VfsWatcher> {
    //     self.watcher.as_ref()
    // }

    // pub(crate) fn set_watcher(&mut self, watcher: VfsWatcher) {
    //     assert!(self.watcher.is_none());
    //     self.watcher = Some(watcher)
    // }

    pub fn current_dir(&self) -> &Path {
        &self.current_dir
    }
}
