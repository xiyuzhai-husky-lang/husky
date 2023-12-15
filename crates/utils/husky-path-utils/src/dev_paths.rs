use crate::*;
use std::path::{Path, PathBuf};

/// paths useful for the development of the Husky programming language
pub struct HuskyLangDevPaths {
    cargo_manifest_dir: Option<PathBuf>,
    lang_dev_root: PathBuf,
    /// the path for dev library
    lang_dev_library_dir: PathBuf,
    /// the path for dev examples
    lang_dev_examples_dir: PathBuf,
    /// the path for dev registry
    lang_dev_registry_dir: PathBuf,
}

impl HuskyLangDevPaths {
    pub fn new() -> Self {
        let cargo_manifest_dir: Option<PathBuf> = std::env::var("CARGO_MANIFEST_DIR")
            .ok()
            .map(|path| path.into());
        let lang_dev_root = find_lang_dev_root().expect("todo");
        let lang_dev_library_dir = lang_dev_root.join("library");
        let lang_dev_examples_dir = lang_dev_root.join("examples");
        let lang_dev_registry_dir = lang_dev_root.join("registry");
        Self {
            cargo_manifest_dir,
            lang_dev_root,
            lang_dev_library_dir,
            lang_dev_examples_dir,
            lang_dev_registry_dir,
        }
    }

    pub fn cargo_manifest_dir(&self) -> Option<&Path> {
        self.cargo_manifest_dir.as_ref().map(|path| path as &Path)
    }

    pub fn dev_root(&self) -> &PathBuf {
        &self.lang_dev_root
    }

    pub fn lang_dev_library_dir(&self) -> &PathBuf {
        &self.lang_dev_library_dir
    }

    pub fn lang_dev_examples_dir(&self) -> &PathBuf {
        &self.lang_dev_examples_dir
    }

    pub fn lang_dev_registry_dir(&self) -> &PathBuf {
        &self.lang_dev_registry_dir
    }
}
