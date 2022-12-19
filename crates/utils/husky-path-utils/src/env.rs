use std::path::{Path, PathBuf};

pub struct HuskyDevPathEnv {
    cargo_manifest_dir: PathBuf,
    lang_dev_root: PathBuf,
    lang_dev_library_dir: PathBuf,
    lang_dev_examples_dir: PathBuf,
}

impl HuskyDevPathEnv {
    pub fn new() -> Self {
        let cargo_manifest_dir: PathBuf = std::env::var("CARGO_MANIFEST_DIR").unwrap().into();
        let dev_root = {
            let mut dir: &Path = &cargo_manifest_dir;
            loop {
                if dir.join("husky-toolchain.toml").exists() {
                    assert!(dir.join("husky-toolchain.toml").is_file());
                    assert!(dir.join("corgi-config.toml").exists());
                    assert!(dir.join("corgi-config.toml").is_file());
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
        };
        let dev_library_dir = dev_root.join("library");
        let dev_examples_dir = dev_root.join("examples");
        Self {
            cargo_manifest_dir,
            lang_dev_root: dev_root,
            lang_dev_library_dir: dev_library_dir,
            lang_dev_examples_dir: dev_examples_dir,
        }
    }

    pub fn cargo_manifest_dir(&self) -> &PathBuf {
        &self.cargo_manifest_dir
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
}
