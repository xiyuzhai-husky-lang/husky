use crate::*;
use husky_package_path::{PackagePath, PackagePathDb};
use husky_vfs::VfsDb;
use salsa::DbWithJar;

pub trait TomlTokenizeDb: DbWithJar<TomlTokenizeJar> + PackagePathDb + VfsDb {
    fn toml_tokenize(&self, input: &str) -> Vec<TomlToken>;

    fn toml_token_sheet(&self, package: PackagePath) -> TomlTokenSheet;
}

impl<T> TomlTokenizeDb for T
where
    T: DbWithJar<TomlTokenizeJar> + PackagePathDb + VfsDb,
{
    fn toml_tokenize(&self, input: &str) -> Vec<TomlToken> {
        TokenIter::new(self, input).collect()
    }

    fn toml_token_sheet(&self, package: PackagePath) -> TomlTokenSheet {
        package_manifest_toml_token_sheet(self, package)
    }
}
