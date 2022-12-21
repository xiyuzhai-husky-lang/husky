use husky_toolchain::{ToolchainDb, ToolchainJar};
use husky_word::WordJar;

use crate::*;

#[salsa::db(WordJar, ToolchainJar, VfsJar)]
#[derive(Default)]
pub(crate) struct DB {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for DB {}

#[test]
fn package_path_debug_works() {
    let db = DB::default();
    let toolchain = db.lang_dev_toolchain();
    let package_path_menu = db.package_path_menu(toolchain).as_ref().unwrap();
    expect_test::expect![[r#"
        Local {
            path: "/home/xiyuzhai/repos/husky/library/core",
        }
    "#]]
    .assert_debug_eq(&package_path_menu.core().debug(&db));
}
