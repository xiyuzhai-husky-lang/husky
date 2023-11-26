//! Test that a constant `tracked` fn (has no inputs)
//! compiles and executes successfully.
#![allow(warnings)]

use salsa::Db;

#[salsa::jar(db = Db)]
struct Jar(tracked_fn);

#[salsa::tracked]
fn tracked_fn(db: &Db) -> u32 {
    44
}

#[test]
fn execute() {
    #[salsa::db(Jar)]
    struct Database;

    let mut db = Database::default();
    assert_eq!(tracked_fn(&db), 44);
}
