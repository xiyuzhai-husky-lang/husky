use crate::*;
use husky_vfs::VfsDb;

pub trait ManifestDb: DbWithJar<ManifestJar> + VfsDb {
    fn package_dependencies(
        &self,
        package_path: PackagePath,
    ) -> ManifestResult<&[PackageDependency]>;
}

impl<Db> ManifestDb for Db
where
    Db: DbWithJar<ManifestJar> + VfsDb,
{
    fn package_dependencies(
        &self,
        package_path: PackagePath,
    ) -> ManifestResult<&[PackageDependency]> {
        Ok(package_dependencies(self, package_path).as_ref()?)
    }
}
