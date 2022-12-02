use crate::*;
use husky_package_path::{PackagePath, PackagePathDb};
use husky_source_path::{SourcePath, SourcePathData, SourcePathDb};
use husky_toml_token_text::TomlTokenText;
use husky_vfs::{VfsDb, VfsResult};
use salsa::DbWithJar;

pub trait TomlTokenizeDb: DbWithJar<TomlTokenizeJar> + SourcePathDb + VfsDb {
    fn toml_tokenize(&self, input: &str) -> Vec<TomlToken>;

    fn toml_token_text(&self, path: SourcePath) -> &VfsResult<TomlTokenText>;

    fn package_manifest_token_text(&self, package: PackagePath) -> &VfsResult<TomlTokenText>;
}

impl<T> TomlTokenizeDb for T
where
    T: DbWithJar<TomlTokenizeJar> + PackagePathDb + VfsDb,
{
    fn toml_tokenize(&self, input: &str) -> Vec<TomlToken> {
        TomlTokenIter::new(self, input).collect()
    }

    fn toml_token_text(&self, path: SourcePath) -> &VfsResult<TomlTokenText> {
        toml_token_text(self, path)
    }

    fn package_manifest_token_text(&self, package: PackagePath) -> &VfsResult<TomlTokenText> {
        self.toml_token_text(self.it_source_path(SourcePathData::CorgiToml(package)))
    }
}
