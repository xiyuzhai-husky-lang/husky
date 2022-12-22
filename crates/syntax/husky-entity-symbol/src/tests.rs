use crate::*;

use husky_ast::AstJar;
use husky_entity_path::EntityPathJar;
use husky_token::TokenJar;
use husky_vfs::*;
use husky_word::WordJar;

#[salsa::db(WordJar, VfsJar, EntityPathJar, TokenJar, AstJar, EntitySymbolJar)]
#[derive(Default)]
pub(crate) struct DB {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for DB {}
