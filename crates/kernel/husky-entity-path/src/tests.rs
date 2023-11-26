use crate::*;
use husky_coword::CowordJar;
use salsa::{database::DatabaseDyn, DebugWithDb};

#[salsa::test_db(CowordJar, VfsJar, EntityPathJar)]
#[derive(Default)]
pub(crate) struct DB;

#[test]
fn item_path_debug_works() {
    let db = DB::default();
    let db = &*db;
    let toolchain = db.dev_toolchain().unwrap();
    let item_path_menu = item_path_menu(db, toolchain);
    expect_test::expect![[r#"
        TypePath(`core::num::i32`, `Extern`)
    "#]]
    .assert_debug_eq(&item_path_menu.i32_ty_path().debug(db));
}
