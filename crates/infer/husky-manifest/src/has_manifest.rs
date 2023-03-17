use crate::*;

pub trait HasManifest {
    fn manifest_dependencies(self, db: &dyn ManifestDb) -> ManifestResult<&[ManifestDependency]>;
}

impl HasManifest for PackagePath {
    fn manifest_dependencies(self, db: &dyn ManifestDb) -> ManifestResult<&[ManifestDependency]> {
        Ok(manifest_dependencies(db, self).as_ref()?)
    }
}
