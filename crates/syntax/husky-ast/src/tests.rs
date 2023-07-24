pub(crate) use husky_token::test_utils::*;

use crate::*;
use husky_coword::CowordJar;
use husky_item_path::EntityPathJar;
use husky_term_prelude::TermPreludeJar;
use husky_token::TokenJar;
use husky_vfs::*;
use salsa::{Database, ParallelDatabase, Snapshot};

#[salsa::db(CowordJar, VfsJar, EntityPathJar, TokenJar, AstJar, TermPreludeJar)]
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
    let mut db = DB::default();
    db.token_expect_test_debug_with_db("ast_sheet", AstDb::ast_sheet);
}
