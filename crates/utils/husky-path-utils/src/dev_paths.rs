use crate::*;
use std::path::{Path, PathBuf};

/// paths useful for the development of the Husky programming language
pub struct HuskyLangDevPaths {
    cargo_manifest_dir: Option<PathBuf>,
    root: PathBuf,
    /// the path for dev library
    library_dir: PathBuf,
    /// the path for dev examples
    examples_dir: PathBuf,
    /// the path for experiments
    experiments_dir: PathBuf,
    projects_dir: PathBuf,
    /// the path for dev registry
    registry_dir: PathBuf,
    sessions_dir: PathBuf,
    specs_dir: PathBuf,
    python_lib_dir: PathBuf,
    lean4_lib_dir: PathBuf,
    rocq_lib_dir: PathBuf,
    isabelle_lib_dir: PathBuf,
}

impl HuskyLangDevPaths {
    pub fn new() -> Self {
        let cargo_manifest_dir: Option<PathBuf> = std::env::var("CARGO_MANIFEST_DIR")
            .ok()
            .map(|path| path.into());
        let root = find_lang_dev_root().expect("todo");
        let library_dir = root.join("library");
        let examples_dir = root.join("examples");
        let projects_dir = root.join("projects");
        let registry_dir = root.join("registry");
        let experiments_dir = root.join("experiments");
        let sessions_dir = root.join("sessions");
        let specs_dir = root.join("specs");
        let python_lib_dir = root.join("python");
        let lean4_lib_dir = root.join("lean4");
        let rocq_lib_dir = root.join("rocq");
        let isabelle_lib_dir = root.join("isabelle");
        Self {
            cargo_manifest_dir,
            root,
            library_dir,
            examples_dir,
            experiments_dir,
            projects_dir,
            registry_dir,
            sessions_dir,
            specs_dir,
            python_lib_dir,
            lean4_lib_dir,
            rocq_lib_dir,
            isabelle_lib_dir,
        }
    }

    pub fn cargo_manifest_dir(&self) -> Option<&Path> {
        self.cargo_manifest_dir.as_ref().map(|path| path as &Path)
    }

    pub fn root(&self) -> &PathBuf {
        &self.root
    }

    pub fn library_dir(&self) -> &PathBuf {
        &self.library_dir
    }

    pub fn examples_dir(&self) -> &PathBuf {
        &self.examples_dir
    }

    pub fn experiments_dir(&self) -> &PathBuf {
        &self.experiments_dir
    }

    pub fn projects_dir(&self) -> &PathBuf {
        &self.projects_dir
    }

    pub fn registry_dir(&self) -> &PathBuf {
        &self.registry_dir
    }

    pub fn sessions_dir(&self) -> &PathBuf {
        &self.sessions_dir
    }

    pub fn specs_dir(&self) -> &PathBuf {
        &self.specs_dir
    }

    pub fn python_lib_dir(&self) -> &PathBuf {
        &self.python_lib_dir
    }

    pub fn lean4_lib_dir(&self) -> &PathBuf {
        &self.lean4_lib_dir
    }

    pub fn rocq_lib_dir(&self) -> &PathBuf {
        &self.rocq_lib_dir
    }

    pub fn isabelle_lib_dir(&self) -> &PathBuf {
        &self.isabelle_lib_dir
    }
}
