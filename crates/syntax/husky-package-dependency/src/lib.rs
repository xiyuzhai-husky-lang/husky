mod db;
mod error;

pub use db::*;
pub use error::*;

use husky_package_path::PackagePath;
use husky_vfs::VfsResult;

#[salsa::jar(db = PackageDependencyDb)]
pub struct PackageDependencyJar(unchecked_package_dependencies, package_dependencies);

#[salsa::tracked(jar = PackageDependencyJar, return_ref)]
fn unchecked_package_dependencies(
    db: &dyn PackageDependencyDb,
    package_path: PackagePath,
) -> VfsResult<Vec<PackageDependency>> {
    todo!()
}

#[salsa::tracked(jar = PackageDependencyJar, return_ref)]
fn package_dependencies(
    db: &dyn PackageDependencyDb,
    package_path: PackagePath,
) -> PackageDependencyResult<Vec<PackageDependency>> {
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
