use crate::*;
use husky_toml_token::TomlTokenDb;
use salsa::DbWithJar;

pub trait TomlAstDb: DbWithJar<TomlAstJar> + TomlTokenDb {
    fn package_manifest_toml_ast(&self, package: PackagePath) -> VfsResult<&TomlAstSheet>;
}

impl<Db> TomlAstDb for Db
where
    Db: DbWithJar<TomlAstJar> + TomlTokenDb,
{
    fn package_manifest_toml_ast(&self, package: PackagePath) -> VfsResult<&TomlAstSheet> {
        Ok(package_manifest_toml_ast(self, package).as_ref()?)
    }
}
