use crate::*;
use husky_vfs::VfsDb;

pub trait ManifestDb: DbWithJar<ManifestJar> + VfsDb {
    fn package_dependencies(
        &self,
        package_path: PackagePath,
    ) -> ManifestResult<&[PackageDependency]>;
}

impl<DB> ManifestDb for DB
where
    DB: DbWithJar<ManifestJar> + VfsDb,
{
    fn package_dependencies(
        &self,
        package_path: PackagePath,
    ) -> ManifestResult<&[PackageDependency]> {
        Ok(package_dependencies(self, package_path).as_ref()?)
    }
}
