use crate::*;
use husky_word::WordJar;
use salsa::DebugWithDb;

#[salsa::db(WordJar, VfsJar, EntityPathJar)]
#[derive(Default)]
pub(crate) struct DB {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for DB {}

#[test]
fn entity_path_debug_works() {
    let db = DB::default();
    let toolchain = db.dev_toolchain().unwrap();
    let entity_path_menu = db.entity_path_menu(toolchain);
    expect_test::expect![[r#"
        TypePath(`core::num::i32`, `Extern`)
    "#]]
    .assert_debug_eq(&entity_path_menu.i32_ty_path().debug(&db));
}
