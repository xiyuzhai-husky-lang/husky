use crate::*;
use expect_test::expect_file;
use husky_expect_test_utils::*;

use husky_vfs::*;
use husky_word::WordJar;
use salsa::{Database, Storage};

#[salsa::db(WordJar, VfsJar, TokenJar)]
#[derive(Default)]
struct MimicDB {
    storage: Storage<Self>,
}

impl Database for MimicDB {}

#[test]
fn tokenize_works() {
    expect_test_husky_to_rust("batch", &tokenize_debug);

    fn tokenize_debug(text: &str) -> String {
        format!("{:#?}", MimicDB::default().tokenize(text))
    }
}

#[test]
fn tokenize_library() {
    let db = MimicDB::default();
    let toolchain = db.lang_dev_toolchain();
    let path_menu = db.path_menu(toolchain).unwrap();

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
