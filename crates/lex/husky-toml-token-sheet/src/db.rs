use crate::*;
use husky_package_path::{PackagePath, PackagePathDb};
use husky_toml_tokenize::TomlTokenizeDb;
use salsa::DbWithJar;

pub trait TomlTokenSheetDb: DbWithJar<TomlTokenSheetJar> + PackagePathDb + TomlTokenizeDb {
    fn toml_token_sheet(&self, package: PackagePath) -> TomlTokenSheet;
}

impl<T> TomlTokenSheetDb for T
where
    T: DbWithJar<TomlTokenSheetJar> + PackagePathDb + TomlTokenizeDb,
{
    fn toml_token_sheet(&self, package: PackagePath) -> TomlTokenSheet {
        toml_token_sheet(self, package)
    }
}
