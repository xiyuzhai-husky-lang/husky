use crate::*;

pub trait HasPackageManifest: Copy {
    fn package_manifest(self, db: &dyn ManifestDb) -> ManifestResultRef<PackageManifest>;

    fn package_deps(self, db: &dyn ManifestDb) -> ManifestResultRef<&[ManifestDependency]>;
}

impl HasPackageManifest for PackagePath {
    fn package_manifest(self, db: &dyn ManifestDb) -> ManifestResultRef<PackageManifest> {
        package_manifest(db, self)
    }

    fn package_deps(self, db: &dyn ManifestDb) -> ManifestResultRef<&[ManifestDependency]> {
        // is this necessary for keeping things as lazy as possible?
        package_dependencies(db, self)
    }
}
