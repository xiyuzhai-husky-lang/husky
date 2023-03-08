use std::sync::RwLock;

use crate::*;
use husky_fs_specs::{corgi_install_path, huskyup_install_path, FsSpecsError, FsSpecsResult};
use vec_like::VecSet;

pub struct VfsCache {
    files: DashMap<PathBuf, File>,
    live_packages: RwLock<VecSet<PackagePath>>,
    corgi_install_path: FsSpecsResult<PathBuf>,
    huskyup_install_path: FsSpecsResult<PathBuf>,
    base_path: VfsResult<PathBuf>,
    watcher: Option<VfsWatcher>,
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
            live_packages: Default::default(),
            corgi_install_path,
            huskyup_install_path,
            base_path: match std::env::current_dir() {
                Ok(dir) => std::path::absolute(dir).map_err(|_e| todo!()),
                Err(_e) => todo!(),
            },
            watcher: None,
        }
    }
}

impl VfsCache {
    pub(crate) fn files(&self) -> &DashMap<PathBuf, File> {
        &self.files
    }

    pub(crate) fn live_packages<'a>(
        &'a self,
    ) -> std::sync::LockResult<std::sync::RwLockReadGuard<'_, VecSet<PackagePath>>> {
        self.live_packages.read()
    }

    pub fn corgi_install_path(&self) -> Result<&PathBuf, &FsSpecsError> {
        self.corgi_install_path.as_ref()
    }

    pub fn huskyup_install_path(&self) -> Result<&PathBuf, &FsSpecsError> {
        self.huskyup_install_path.as_ref()
    }

    pub fn watcher(&self) -> Option<&VfsWatcher> {
        self.watcher.as_ref()
    }

    pub(crate) fn set_watcher(&mut self, watcher: VfsWatcher) {
        assert!(self.watcher.is_none());
        self.watcher = Some(watcher)
    }

    pub fn base_path(&self) -> Result<&PathBuf, &VfsError> {
        self.base_path.as_ref()
    }

    pub(crate) fn add_live_package(&self, package_path: PackagePath) {
        match self.live_packages.write() {
            Ok(mut live_packages) => live_packages.insert(package_path),
            Err(_) => todo!(),
        }
    }
}
