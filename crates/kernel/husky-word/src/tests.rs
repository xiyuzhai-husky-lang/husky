use salsa::DebugWithDb;

use crate::*;

#[salsa::db(WordJar)]
#[derive(Default)]
pub struct DB {
    storage: salsa::Storage<DB>,
}

impl salsa::Database for DB {}

impl salsa::ParallelDatabase for DB {
    fn snapshot(&self) -> salsa::Snapshot<Self> {
        salsa::Snapshot::new(DB {
            storage: self.storage.snapshot(),
        })
    }
}

#[test]
fn word_debug_works() {
    let db = DB::default();
    let haha = db.it_word_borrowed("haha");
    expect_test::expect![[r#"
        Word(
            "haha",
        )
    "#]]
    .assert_debug_eq(&haha.debug(&db));
}

#[test]
fn ident_debug_works() {
    let db = DB::default();
    let haha = db.it_ident_borrowed("haha").unwrap();
    expect_test::expect![[r#"
        "haha"
    "#]]
    .assert_debug_eq(&haha.debug(&db));
}
