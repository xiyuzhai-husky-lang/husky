use crate::*;
use expect_test::expect_file;
use husky_entity_path::{EntityPathDb, EntityPathJar};
use husky_expect_test_utils::*;
use husky_package_path::{PackagePathDb, PackagePathJar};

use husky_toolchain::*;
use husky_vfs::*;
use husky_word::WordJar;
use salsa::{Database, Storage};

#[salsa::db(WordJar, ToolchainJar, PackagePathJar, TokenJar, VfsJar, EntityPathJar)]
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
    let _package_path_menu = db.package_path_menu(toolchain).as_ref().unwrap();
    let entity_path_menu = db.entity_path_menu(toolchain).as_ref().unwrap();

    macro_rules! t {
        ($($module:ident),*) => {
            $(
                expect_file![format!("../tests/single/{}_token_sheet.txt", stringify!($module))]
                    .assert_eq(&format!("{:#?}", db.token_sheet(entity_path_menu.$module())))
            );*
        }
    }

    t!(core, core_basic, core_num, std);
}
