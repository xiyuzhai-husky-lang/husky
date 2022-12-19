use crate::*;
use husky_package_path::PackagePathDb;
use salsa::DbWithJar;

pub trait PackageDependencyDb: DbWithJar<PackageDependencyJar> + PackagePathDb {
    fn package_dependencies(
        &self,
        package_path: PackagePath,
    ) -> &PackageDependencyResult<Vec<PackageDependency>>;
}

impl<T> PackageDependencyDb for T
where
    T: DbWithJar<PackageDependencyJar> + PackagePathDb,
{
    fn package_dependencies(
        &self,
        package_path: PackagePath,
    ) -> &PackageDependencyResult<Vec<PackageDependency>> {
        package_dependencies(self, package_path)
    }
}
