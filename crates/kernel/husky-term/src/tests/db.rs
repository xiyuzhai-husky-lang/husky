use crate::*;
use husky_entity_path::EntityPathJar;

use husky_word::WordJar;

#[salsa::db(TermJar, EntityPathJar, VfsJar, WordJar)]
#[derive(Default)]
pub(crate) struct TermTestsDb {
    storage: salsa::Storage<TermTestsDb>,
}

impl salsa::Database for TermTestsDb {}
