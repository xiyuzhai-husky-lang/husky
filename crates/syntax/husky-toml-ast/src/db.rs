use crate::*;
use husky_source_path::SourcePathDb;
use husky_toml_tokenize::TomlTokenizeDb;
use salsa::DbWithJar;

pub trait TomlAstDb: DbWithJar<TomlAstJar> + TomlTokenizeDb + SourcePathDb {}
