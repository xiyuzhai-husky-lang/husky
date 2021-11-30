mod discover_projects;
mod manifest_path;

pub use crate::manifest_path::ManifestPath;
pub use discover_projects::discover_projects;

use common::error::*;
use vfs::AbsPathBuf;

pub enum ProjectManifest {
    ProjectJson(ManifestPath),
    CargoToml(ManifestPath),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Project {
    main: AbsPathBuf,
}

impl Project {
    pub fn new(path: AbsPathBuf) -> Result<Project> {
        Ok(Project {
            main: path.to_path_buf(),
        })
    }
}
