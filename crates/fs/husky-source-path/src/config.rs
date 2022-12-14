use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use husky_print_utils::p;

#[derive(Debug)]
pub struct SourcePathConfig {
    library_dir: PathBuf,
}

impl SourcePathConfig {
    pub fn library_dir(&self) -> &Path {
        &self.library_dir
    }
}

pub trait HasSourcePathConfig {
    fn source_path_config(&self) -> &SourcePathConfig;
}

#[derive(Debug, Clone)]
pub struct SourcePathConfigImpl(Arc<SourcePathConfig>);

impl Default for SourcePathConfigImpl {
    fn default() -> Self {
        Self(Arc::new(SourcePathConfig {
            // ad hoc
            library_dir: "/home/xiyuzhai/repos/husky/library".into(),
        }))
    }
}

#[derive(Debug, Clone)]
pub struct SourcePathConfigMimic(Arc<SourcePathConfig>);

impl std::ops::Deref for SourcePathConfigImpl {
    type Target = SourcePathConfig;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::Deref for SourcePathConfigMimic {
    type Target = SourcePathConfig;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for SourcePathConfigMimic {
    fn default() -> Self {
        Self(Arc::new(SourcePathConfig {
            library_dir: derive_library_dir_from_cargo_manifest_dir(
                std::env::var("CARGO_MANIFEST_DIR").unwrap(),
            ),
        }))
    }
}

fn derive_library_dir_from_cargo_manifest_dir(cargo_manifest_dir: impl AsRef<Path>) -> PathBuf {
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
