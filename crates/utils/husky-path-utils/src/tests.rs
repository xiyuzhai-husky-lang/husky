use husky_word::WordJar;

#[salsa::db(WordJar)]
#[derive(Default)]
pub(crate) struct DB {
    storage: salsa::Storage<DB>,
}

impl salsa::Database for DB {}
