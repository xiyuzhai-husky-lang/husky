use crate::*;
use husky_word::WordJar;

#[salsa::db(VfsJar, WordJar, TokenJar, TokenInferJar)]
pub(crate) struct DB {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for DB {}
