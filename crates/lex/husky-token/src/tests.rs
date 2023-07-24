mod adv;

use crate::*;
use husky_coword::CowordJar;
use husky_expect_test_snippets_utils::*;
use husky_item_path::EntityPathJar;
use husky_vfs::*;
use salsa::{Database, DebugWithDb, Storage};

#[salsa::db(CowordJar, VfsJar, TokenJar, TermPreludeJar, EntityPathJar)]
#[derive(Default)]
pub(crate) struct DB {
    storage: Storage<Self>,
}

impl Database for DB {}

fn tokenize_snippet_debug(snippet: &str) -> String {
    let db = DB::default();
    let snippet = Snippet::new(&db, snippet.to_owned());
    format!("{:#?}", db.snippet_token_sheet_data(snippet).debug(&db))
}

#[test]
fn tokenize_works() {
    expect_test_snippets("snippets", &tokenize_snippet_debug);
}

#[test]
fn token_sheet_works() {
    DB::default().vfs_expect_test_debug_with_db("token_sheet", TokenDb::token_sheet_data)
}
