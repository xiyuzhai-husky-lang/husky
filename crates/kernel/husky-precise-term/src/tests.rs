use husky_entity_path::EntityPathJar;
use husky_raw_term::RawTermJar;
use husky_word::WordJar;

#[salsa::db(TermJar, EntityPathJar, VfsJar, WordJar, RawTermJar)]
#[derive(Default)]
pub(crate) struct DB {
    storage: salsa::Storage<DB>,
}

impl salsa::Database for DB {}
