use crate::*;
use salsa::DebugWithDb;

#[salsa::test_db(CowordJar)]
pub struct DB;

#[test]
fn word_debug_works() {
    let db = DB::default();
    let db = &*db;
    let haha = db.it_coword_borrowed("haha");
    expect_test::expect![[r#"
        Word(
            "haha",
        )
    "#]]
    .assert_debug_eq(&haha.debug(db));
}

#[test]
fn ident_debug_works() {
    let db = DB::default();
    let db = &*db;
    let haha = db.it_ident_borrowed("haha").unwrap();
    expect_test::expect![[r#"
        Ident(
            "haha",
        )
    "#]]
    .assert_debug_eq(&haha.debug(db));
}
