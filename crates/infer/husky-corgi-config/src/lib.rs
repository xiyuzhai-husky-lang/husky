mod db;

pub use db::*;

use husky_vfs::*;

#[salsa::jar(db = CorgiConfigDb)]
pub struct CorgiConfigJar(package_registry_path, CorgiConfig);

#[salsa::tracked(db = CorgiConfigDb, jar = CorgiConfigJar)]
pub struct CorgiConfig {}

#[salsa::tracked(jar = CorgiConfigJar)]
pub(crate) fn package_registry_path(
    db: &dyn CorgiConfigDb,
    package: PackagePath,
) -> VfsResult<RegistryPath> {
    let corgi_config = package.corgi_config(db);
    todo!()
}

pub trait HasCorgiConfig: Copy {
    fn corgi_config(self, db: &dyn CorgiConfigDb) -> VfsResult<CorgiConfig>;

    fn registry_path(self, db: &dyn CorgiConfigDb) -> VfsResult<RegistryPath>;
}

impl HasCorgiConfig for PackagePath {
    fn corgi_config(self, db: &dyn CorgiConfigDb) -> VfsResult<CorgiConfig> {
        todo!()
    }

    fn registry_path(self, db: &dyn CorgiConfigDb) -> VfsResult<RegistryPath> {
        package_registry_path(db, self)
    }
}
