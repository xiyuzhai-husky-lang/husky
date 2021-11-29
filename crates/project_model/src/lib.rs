mod manifest_path;

pub use crate::manifest_path::ManifestPath;

pub enum ProjectManifest {
    ProjectJson(ManifestPath),
    CargoToml(ManifestPath),
}

#[derive(Debug)]
pub struct Project {}
