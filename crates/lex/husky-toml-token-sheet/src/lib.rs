mod db;
mod storage;
#[cfg(test)]
mod tests;

pub use db::*;
pub use storage::*;

use husky_package_path::PackagePath;
use husky_toml_token::TomlToken;

#[salsa::jar(db = TomlTokenSheetDb)]
pub struct TomlTokenSheetJar(TomlTokenSheet);

#[salsa::tracked(jar =  TomlTokenSheetJar)]
pub struct TomlTokenSheet {
    #[return_ref]
    tokens: TomlTokens,
}

fn toml_token_sheet(db: &dyn TomlTokenSheetDb, package: PackagePath) -> TomlTokenSheet {
    todo!()
}
