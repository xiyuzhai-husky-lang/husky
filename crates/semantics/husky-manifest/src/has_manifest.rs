use crate::*;

pub trait HasPackageManifest: Copy {
    fn package_manifest(self, db: &::salsa::Db) -> ManifestResultRef<PackageManifest>;

    fn package_dependencies(self, db: &::salsa::Db) -> ManifestResultRef<&[PackageDependency]>;
}

impl HasPackageManifest for PackagePath {
    fn package_manifest(self, db: &::salsa::Db) -> ManifestResultRef<PackageManifest> {
        package_manifest(db, self)
    }

    fn package_dependencies(self, db: &::salsa::Db) -> ManifestResultRef<&[PackageDependency]> {
        // is this necessary for keeping things as lazy as possible?
        package_dependencies(db, self)
    }
}
