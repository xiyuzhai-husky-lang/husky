use crate::*;

pub trait HasPackageManifest: Copy {
    fn package_manifest(self, db: &dyn ManifestDb) -> VfsResult<PackageManifest>;

    fn package_dependencies(self, db: &dyn ManifestDb) -> VfsResult<&[PackageDependency]>;
}

impl HasPackageManifest for PackagePath {
    fn package_manifest(self, db: &dyn ManifestDb) -> VfsResult<PackageManifest> {
        package_manifest(db, self)
    }

    fn package_dependencies(self, db: &dyn ManifestDb) -> VfsResult<&[PackageDependency]> {
        // is this necessary for keeping things as lazy as possible?
        Ok(package_dependencies(db, self)?.data(db))
    }
}
