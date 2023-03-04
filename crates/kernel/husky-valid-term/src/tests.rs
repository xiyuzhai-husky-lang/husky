use crate::*;
use husky_entity_path::EntityPathJar;
use husky_precise_term::PreciseTermJar;
use husky_raw_term::RawTermJar;
use husky_vfs::VfsJar;
use husky_word::WordJar;

#[salsa::db(
    EntityPathJar,
    VfsJar,
    WordJar,
    RawTermJar,
    PreciseTermJar,
    ValidTermJar
)]
#[derive(Default)]
pub(crate) struct DB {
    storage: salsa::Storage<DB>,
}

impl salsa::Database for DB {}
