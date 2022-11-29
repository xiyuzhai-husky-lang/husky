mod db;

pub use db::*;

#[salsa::jar(db = TomlTokenSheetDb)]
pub struct TomlTokenSheetJar(TomlTokenSheet);

#[salsa::tracked(jar =  TomlTokenSheetJar)]
pub struct TomlTokenSheet {}

#[cfg(test)]
mod tests;
