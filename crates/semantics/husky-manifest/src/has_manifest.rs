use husky_vfs::linktime_target_path::{LinktimeTargetPath, LinktimeTargetPathData};
use vec_like::VecSet;

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
    }

    /// includes package_path itself
    fn full_dependencies(self, db: &salsa::Db) -> ManifestResultRef<&[PackagePath]> {
        full_dependent_package_paths(db, self)
    }
}

pub trait HasAllPackages: Copy {
    fn all_packages(self, db: &::salsa::Db) -> ManifestResultRef<&[PackagePath]>;
}

impl HasAllPackages for LinktimeTargetPath {
    fn all_packages(self, db: &salsa::Db) -> ManifestResultRef<&[PackagePath]> {
        linktime_target_path_all_packages(db, self)
            .as_ref()
            .map(|v| v as &[_])
    }
}

#[salsa::tracked(jar = ManifestJar, return_ref)]
fn linktime_target_path_all_packages(
    db: &::salsa::Db,
    target_path: LinktimeTargetPath,
) -> ManifestResult<VecSet<PackagePath>> {
    Ok(match target_path.data(db) {
        LinktimeTargetPathData::Package(package_path) => {
            VecSet::from_iter(package_path.full_dependencies(db)?.iter().copied())
        }
        LinktimeTargetPathData::Workspace(_) => todo!(),
    })
}
