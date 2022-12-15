use crate::*;
use husky_package_path::PackagePath;
use husky_toml_token::TomlTokenDb;
use salsa::DbWithJar;

pub trait TomlAstDb: DbWithJar<TomlAstJar> + TomlTokenDb {
    fn package_manifest_ast(&self, package: PackagePath) -> &VfsResult<TomlAst>;
}

impl<T> TomlAstDb for T
where
    T: DbWithJar<TomlAstJar> + TomlTokenDb,
{
    fn package_manifest_ast(&self, package: PackagePath) -> &VfsResult<TomlAst> {
        package_manifest_ast(self, package)
    }
}
