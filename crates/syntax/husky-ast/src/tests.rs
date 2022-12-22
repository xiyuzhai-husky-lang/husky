use crate::*;
use husky_entity_path::EntityPathJar;
use husky_token::TokenJar;
use husky_vfs::*;
use husky_word::WordJar;
use salsa::{Database, ParallelDatabase, Snapshot};

#[salsa::db(WordJar, VfsJar, EntityPathJar, TokenJar, AstJar)]
#[derive(Default)]
pub(crate) struct DB {
    storage: salsa::Storage<Self>,
}

impl Database for DB {}

impl ParallelDatabase for DB {
    fn snapshot(&self) -> Snapshot<Self> {
        Snapshot::new(DB {
            storage: self.storage.snapshot(),
        })
    }
}

#[test]
fn ast_sheet_works() {
    use tests::*;
    DB::expect_test_probable_modules_debug_result_with_db("ast_sheet", AstDb::ast_sheet);
}
