use crate::*;

pub trait HasPackageManifest: Copy {
    fn package_manifest(self, db: &::salsa::Db) -> ManifestResultRef<PackageManifest>;

    fn dependencies(self, db: &::salsa::Db) -> ManifestResultRef<&[PackageDependency]>;

    fn full_dependencies(self, db: &::salsa::Db) -> ManifestResultRef<&[PackagePath]>;
}

impl HasPackageManifest for PackagePath {
    fn package_manifest(self, db: &::salsa::Db) -> ManifestResultRef<PackageManifest> {
        package_manifest(db, self)
    }

    fn dependencies(self, db: &::salsa::Db) -> ManifestResultRef<&[PackageDependency]> {
        // is this necessary for keeping things as lazy as possible?
        package_dependencies(db, self)
            .as_ref()
            .map(|section| section.data(db) as &[_])
    }

    /// includes package_path itself
    fn full_dependencies(self, db: &salsa::Db) -> ManifestResultRef<&[PackagePath]> {
        full_dependent_package_paths(db, self)
    }
}
