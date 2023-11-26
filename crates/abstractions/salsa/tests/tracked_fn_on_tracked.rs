//! Test that a `tracked` fn on a `salsa::input`
//! compiles and executes successfully.

use salsa::Db;

#[salsa::jar(db = Db)]
struct Jar(MyInput, MyTracked, tracked_fn);

#[salsa::input(db = Db, jar = Jar)]
struct MyInput {
    field: u32,
}

#[salsa::tracked(db = Db, jar = Jar)]
struct MyTracked {
    field: u32,
}

#[salsa::tracked(jar = Jar)]
fn tracked_fn(db: &Db, input: MyInput) -> MyTracked {
    MyTracked::new(db, input.field(db) * 2)
}

#[salsa::db(Jar)]
#[derive(Default)]
struct Database;

#[test]
fn execute() {
    let db = Database::default();
    let input = MyInput::new(&db, 22);
    assert_eq!(tracked_fn(&db, input).field(&db), 44);
}
