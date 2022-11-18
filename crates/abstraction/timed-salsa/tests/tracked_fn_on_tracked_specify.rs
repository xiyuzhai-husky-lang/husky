//! Test that a `tracked` fn on a `timed_salsa::input`
//! compiles and executes successfully.
#![allow(warnings)]

#[timed_salsa::jar(db = Db)]
struct Jar(MyInput, MyTracked, tracked_fn, tracked_fn_extra);

trait Db: timed_salsa::DbWithJar<Jar> {}

#[timed_salsa::input(jar = Jar)]
struct MyInput {
    field: u32,
}

#[timed_salsa::tracked(jar = Jar)]
struct MyTracked {
    field: u32,
}

#[timed_salsa::tracked(jar = Jar)]
fn tracked_fn(db: &dyn Db, input: MyInput) -> MyTracked {
    let t = MyTracked::new(db, input.field(db) * 2);
    if input.field(db) != 0 {
        tracked_fn_extra::specify(db, t, 2222);
    }
    t
}

#[timed_salsa::tracked(jar = Jar, specify)]
fn tracked_fn_extra(_db: &dyn Db, _input: MyTracked) -> u32 {
    0
}

#[timed_salsa::db(Jar)]
#[derive(Default)]
struct Database {
    storage: timed_salsa::Storage<Self>,
}

impl timed_salsa::Database for Database {}

impl Db for Database {}

#[test]
fn execute_when_specified() {
    let mut db = Database::default();
    let input = MyInput::new(&db, 22);
    let tracked = tracked_fn(&db, input);
    assert_eq!(tracked.field(&db), 44);
    assert_eq!(tracked_fn_extra(&db, tracked), 2222);
}

#[test]
fn execute_when_not_specified() {
    let mut db = Database::default();
    let input = MyInput::new(&db, 0);
    let tracked = tracked_fn(&db, input);
    assert_eq!(tracked.field(&db), 0);
    assert_eq!(tracked_fn_extra(&db, tracked), 0);
}
