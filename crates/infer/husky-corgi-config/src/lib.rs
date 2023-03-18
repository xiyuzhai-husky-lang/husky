mod db;

pub use db::*;

use husky_vfs::*;

#[salsa::jar(db = CorgiConfigDb)]
pub struct CorgiConfigJar(package_registry_path);

#[salsa::tracked(jar = CorgiConfigJar)]
pub(crate) fn package_registry_path(
    db: &dyn CorgiConfigDb,
    package: PackagePath,
) -> VfsResult<RegistryPath> {
    let corgi_config = todo!();
    todo!()
}

pub trait HasCorgiConfig: Copy {
    fn registry_path(self, db: &dyn CorgiConfigDb) -> VfsResult<RegistryPath>;
}

impl HasCorgiConfig for PackagePath {
    fn registry_path(self, db: &dyn CorgiConfigDb) -> VfsResult<RegistryPath> {
        package_registry_path(db, self)
    }
}
