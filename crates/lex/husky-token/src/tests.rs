use crate::*;
use expect_test::expect_file;
use husky_expect_test_snippets_utils::*;

use husky_vfs::*;
use husky_word::WordJar;
use salsa::{Database, Storage};

#[salsa::db(WordJar, VfsJar, TokenJar)]
#[derive(Default)]
struct DB {
    storage: Storage<Self>,
}

impl Database for DB {}

#[test]
fn tokenize_works() {
    expect_test_snippets("snippets", &tokenize_debug);

    fn tokenize_debug(text: &str) -> String {
        format!("{:#?}", DB::default().tokenize(text))
    }
}

#[test]
fn token_sheet_works() {
    DB::expect_test_probable_modules_debug("token_sheet", TokenDb::token_sheet)
}

#[test]
fn tokenize_library() {
    let db = DB::default();
    let toolchain = db.dev_toolchain().unwrap();
    let path_menu = db.dev_path_menu().unwrap();

    macro_rules! t {
        ($($module:ident),*) => {
            $(
                expect_file![format!("../tests/single/{}_token_sheet.txt", stringify!($module))]
                    .assert_eq(&format!("{:#?}", db.token_sheet(path_menu.$module())))
            );*
        }
    }

    t!(core, core_basic, core_num, std);
}
