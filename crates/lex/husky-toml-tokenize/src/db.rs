use crate::*;
use husky_package_path::{PackagePath, PackagePathDb};
use husky_toml_token_text::TomlTokenText;
use husky_vfs::{VfsDb, VfsResult};
use salsa::DbWithJar;

pub trait TomlTokenizeDb: DbWithJar<TomlTokenizeJar> + PackagePathDb + VfsDb {
    fn toml_tokenize(&self, input: &str) -> Vec<TomlToken>;

    fn package_manifest_token_text(&self, package: PackagePath) -> &VfsResult<TomlTokenText>;
}

impl<T> TomlTokenizeDb for T
where
    T: DbWithJar<TomlTokenizeJar> + PackagePathDb + VfsDb,
{
    fn toml_tokenize(&self, input: &str) -> Vec<TomlToken> {
        TokenIter::new(self, input).collect()
    }

    fn package_manifest_token_text(&self, package: PackagePath) -> &VfsResult<TomlTokenText> {
        package_manifest_toml_tokens(self, package)
    }
}
