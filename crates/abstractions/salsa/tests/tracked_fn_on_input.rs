//! Test that a `tracked` fn on a `salsa::input`
//! compiles and executes successfully.
#![allow(warnings)]
use salsa::Db;

#[salsa::jar(db = Db)]
struct Jar(MyInput, tracked_fn);

#[salsa::input(db = Db, jar = Jar)]
struct MyInput {
    field: u32,
}

#[salsa::tracked(jar = Jar)]
fn tracked_fn(db: &Db, input: MyInput) -> u32 {
    input.field(db) * 2
}

#[test]
fn execute() {
    #[salsa::db(Jar)]
    struct Database;

    let mut db = Database::default();
    let input = MyInput::new(&db, 22);
    assert_eq!(tracked_fn(&db, input), 44);
}
