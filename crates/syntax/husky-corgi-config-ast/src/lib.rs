mod db;

pub use self::db::*;

use husky_toml_ast::*;

#[salsa::jar(db = CorgiConfigAstDb)]
pub struct CorgiConfigAstJar();
