use crate::*;
use husky_word::WordJar;

#[salsa::db(VfsJar, WordJar, TokenJar, TokenInferJar, SemanticTokenJar)]
pub(crate) struct DB {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for DB {}
