mod adv;

use crate::*;
use expect_test::expect_file;
use husky_expect_test_snippets_utils::*;

use husky_vfs::*;
use husky_word::WordJar;
use salsa::{Database, Storage};

#[salsa::db(WordJar, VfsJar, TokenJar)]
#[derive(Default)]
pub(crate) struct DB {
    storage: Storage<Self>,
}

impl Database for DB {}

fn tokenize_snippet_debug(snippet: &str) -> String {
    format!("{:#?}", DB::default().tokenize(snippet))
}

#[test]
fn tokenize_works() {
    expect_test_snippets("snippets", &tokenize_snippet_debug);
}

#[test]
fn token_sheet_works() {
    DB::expect_test_probable_modules_debug("token_sheet", TokenDb::token_sheet)
}
