pub(crate) use husky_token::test_utils::*;

use crate::*;
use husky_coword::CowordJar;
use husky_entity_path::EntityPathJar;
use husky_term_prelude::TermPreludeJar;
use husky_token::TokenJar;
use husky_vfs::*;
use salsa::{Database, ParallelDatabase, Snapshot};

#[salsa::test_db(
    CowordJar,
    VfsJar,
    EntityPathJar,
    husky_token_data::db::TokenDataJar,
    TokenJar,
    AstJar,
    TermPreludeJar
)]
pub(crate) struct DB;

#[test]
fn ast_sheet_works() {
    use tests::*;
    let mut db = DB::default();
    db.token_expect_test_debug_with_db(
        |db, module_path| db.ast_sheet(module_path),
        &TokenTestConfig::new("ast_sheet"),
    );
}
