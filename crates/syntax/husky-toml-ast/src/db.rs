use crate::*;
use husky_package_path::PackagePath;
use husky_source_path::{SourcePathData, SourcePathDb};
use husky_toml_token::TomlTokenDb;
use salsa::DbWithJar;

pub trait TomlAstDb: DbWithJar<TomlAstJar> + TomlTokenDb + SourcePathDb {
    fn toml_ast(&self, path: SourcePath) -> &VfsResult<TomlAst>;

    fn package_manifest_ast(&self, package: PackagePath) -> &VfsResult<TomlAst>;
}

impl<T> TomlAstDb for T
where
    T: DbWithJar<TomlAstJar> + TomlTokenDb + SourcePathDb,
{
    fn toml_ast(&self, path: SourcePath) -> &VfsResult<TomlAst> {
        toml_ast(self, path)
    }

    fn package_manifest_ast(&self, package: PackagePath) -> &VfsResult<TomlAst> {
        self.toml_ast(self.it_source_path(SourcePathData::CorgiToml(package)))
    }
}
