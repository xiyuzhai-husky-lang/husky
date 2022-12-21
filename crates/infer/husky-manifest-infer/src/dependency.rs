use crate::*;

#[salsa::tracked(jar = ManifestJar, return_ref)]
pub(crate) fn unchecked_package_dependencies(
    _db: &dyn ManifestDb,
    _package_path: PackagePath,
) -> VfsResult<Vec<PackageDependency>> {
    todo!()
}

#[salsa::tracked(jar = ManifestJar, return_ref)]
pub(crate) fn package_dependencies(
    _db: &dyn ManifestDb,
    _package_path: PackagePath,
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
