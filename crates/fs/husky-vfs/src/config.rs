use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use husky_print_utils::p;

#[derive(Debug)]
pub struct VfsConfig {
    library_dir: PathBuf,
    corgi_install_path: Option<PathBuf>,
}

impl VfsConfig {
    pub fn library_dir(&self) -> &Path {
        &self.library_dir
    }

    pub fn corgi_install_path(&self) -> Option<&Path> {
        self.corgi_install_path.as_ref().map(|p| p.as_ref())
    }
}

pub trait HasVfsConfig {
    fn vfs_config(&self) -> &VfsConfig;
}

#[derive(Debug, Clone)]
pub struct VfsConfigImpl(Arc<VfsConfig>);

impl Default for VfsConfigImpl {
    fn default() -> Self {
        Self(Arc::new(VfsConfig {
            // ad hoc
            library_dir: "/home/xiyuzhai/repos/husky/library".into(),
            corgi_install_path: None,
        }))
    }
}

#[derive(Debug, Clone)]
pub struct VfsConfigMimic(Arc<VfsConfig>);

impl std::ops::Deref for VfsConfigImpl {
    type Target = VfsConfig;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::Deref for VfsConfigMimic {
    type Target = VfsConfig;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for VfsConfigMimic {
    fn default() -> Self {
        Self(Arc::new(VfsConfig {
            library_dir: derive_library_dir_from_cargo_manifest_dir(),
            corgi_install_path: None,
        }))
    }
}

fn derive_library_dir_from_cargo_manifest_dir() -> PathBuf {
    let cargo_manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut library_parent_dir: &Path = cargo_manifest_dir.as_ref();
    loop {
        let library_dir = library_parent_dir.join("library");
        if library_dir.exists() {
            break library_dir;
        }
        if let Some(new_library_parent_dir) = library_parent_dir.parent() {
            library_parent_dir = new_library_parent_dir
        } else {
            todo!()
        }
    }
}
