use crate::*;
use husky_absolute_path::AbsolutePath;
use husky_fs_specs::{corgi_install_path, huskyup_install_path, FsSpecsError, FsSpecsResult};
use std::sync::Arc;

pub struct VfsCache {
    files: DashMap<PathBuf, File>,
    corgi_install_path: FsSpecsResult<PathBuf>,
    huskyup_install_path: FsSpecsResult<PathBuf>,
}

impl Default for VfsCache {
    fn default() -> Self {
        let corgi_install_path = corgi_install_path();
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
        }
    }
}

pub enum HuskyFileCacheKind {
    Major,
    Snapshot,
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
}
