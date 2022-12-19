use crate::*;
use husky_package_path::PackagePath;
use husky_vfs::VfsResult;

#[salsa::tracked(jar = ManifestJar, return_ref)]
pub(crate) fn unchecked_package_dependencies(
    db: &dyn ManifestDb,
    package_path: PackagePath,
) -> VfsResult<Vec<PackageDependency>> {
    todo!()
}

#[salsa::tracked(jar = ManifestJar, return_ref)]
pub(crate) fn package_dependencies(
    db: &dyn ManifestDb,
    package_path: PackagePath,
) -> ManifestResult<Vec<PackageDependency>> {
    todo!()
}

#[derive(Debug, PartialEq, Eq)]
pub struct PackageDependency {
    package_path: PackagePath,
}

impl PackageDependency {
    pub fn package_path(&self) -> PackagePath {
        self.package_path
    }
}
