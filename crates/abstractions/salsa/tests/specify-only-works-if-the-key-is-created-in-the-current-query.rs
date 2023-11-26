//! Test that `specify` only works if the key is a tracked struct created in the current query.
//! compilation succeeds but execution panics
#![allow(warnings)]

use salsa::Db;

#[salsa::jar(db = Db)]
struct Jar(
    MyInput,
    MyTracked,
    tracked_fn,
    tracked_fn_extra,
    tracked_struct_created_in_another_query,
);

#[salsa::input(db = Db, jar = Jar)]
struct MyInput {
    field: u32,
}

#[salsa::tracked(db = Db, jar = Jar)]
struct MyTracked {
    field: u32,
}

#[salsa::tracked(jar = Jar)]
fn tracked_struct_created_in_another_query(db: &Db, input: MyInput) -> MyTracked {
    MyTracked::new(db, input.field(db) * 2)
}

#[salsa::tracked(jar = Jar)]
fn tracked_fn(db: &Db, input: MyInput) -> MyTracked {
    let t = tracked_struct_created_in_another_query(db, input);
    if input.field(db) != 0 {
        tracked_fn_extra::specify(db, t, 2222);
    }
    t
}

#[salsa::tracked(jar = Jar, specify)]
fn tracked_fn_extra(_db: &Db, _input: MyTracked) -> u32 {
    0
}

#[salsa::db(Jar)]
#[derive(Default)]
struct Database;

#[test]
#[should_panic]
fn execute_when_specified() {
    let mut db = Database::default();
    let input = MyInput::new(&db, 22);
    let tracked = tracked_fn(&db, input);
}
