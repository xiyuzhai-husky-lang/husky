use husky_vfs::path::linktime_target_path::{LinktimeTargetPath, LinktimeTargetPathData};

use crate::*;

pub trait HasManifest: Copy {
    fn manifest(self, db: &::salsa::Db) -> ManifestResultRef<PackageManifest>;

    fn dependencies(self, db: &::salsa::Db) -> ManifestResultRef<&[PackageDependency]>;

    fn full_dependencies(self, db: &::salsa::Db) -> ManifestResultRef<&[PackagePath]>;
}

impl HasManifest for PackagePath {
    fn manifest(self, db: &::salsa::Db) -> ManifestResultRef<PackageManifest> {
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

#[test]
fn package_full_dependencies_works() {
    DB::vfs_rich_test_debug_with_db(
        |db, package_path: PackagePath| package_path.full_dependencies(db),
        &VfsTestConfig::new(
            "package_full_dependencies",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::FS,
        ),
    );
}

impl HasManifest for LinktimeTargetPath {
    fn manifest(self, db: &salsa::Db) -> ManifestResultRef<PackageManifest> {
        todo!()
    }

    fn dependencies(self, db: &salsa::Db) -> ManifestResultRef<&[PackageDependency]> {
        todo!()
    }

    fn full_dependencies(self, db: &salsa::Db) -> ManifestResultRef<&[PackagePath]> {
        match self.data(db) {
            LinktimeTargetPathData::Package(package_path) => package_path.full_dependencies(db),
            LinktimeTargetPathData::Workspace(_) => todo!(),
        }
    }
}
