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
        let lang_dev_root: PathBuf = if let Some(ref cargo_manifest_dir) = cargo_manifest_dir {
            let mut dir: &Path = cargo_manifest_dir;
            loop {
                if dir.join("husky-toolchain.toml").exists() {
                    assert!(dir.join("husky-toolchain.toml").is_file());
                    assert!(dir.join(".corgi/config.toml").exists());
                    assert!(dir.join(".corgi/config.toml").is_file());
                    assert!(dir.join("library").exists());
                    assert!(dir.join("library").is_dir());
                    assert!(dir.join("examples").exists());
                    assert!(dir.join("examples").is_dir());
                    assert!(dir.join("registry").exists());
                    assert!(dir.join("registry").is_dir());
                    break dir.to_owned();
                }
                if let Some(new_library_parent_dir) = dir.parent() {
                    dir = new_library_parent_dir
                } else {
                    todo!()
                }
            }
        } else {
            // ad hoc
            "/home/xiyuzhai/repos/husky".into()
        };
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
