mod db;
#[cfg(test)]
mod tests;

pub use db::*;

use husky_package_path::PackagePath;

#[salsa::jar(db = TomlTokenSheetDb)]
pub struct TomlTokenSheetJar(TomlTokenSheet);

#[salsa::tracked(jar =  TomlTokenSheetJar)]
pub struct TomlTokenSheet {}

fn toml_token_sheet(db: &dyn TomlTokenSheetDb, package: PackagePath) -> TomlTokenSheet {
    todo!()
}
