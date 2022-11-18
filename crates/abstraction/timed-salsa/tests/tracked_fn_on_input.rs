//! Test that a `tracked` fn on a `timed_salsa::input`
//! compiles and executes successfully.
#![allow(warnings)]

#[timed_salsa::jar(db = Db)]
struct Jar(MyInput, tracked_fn);

trait Db: timed_salsa::DbWithJar<Jar> {}

#[timed_salsa::input(jar = Jar)]
struct MyInput {
    field: u32,
}

#[timed_salsa::tracked(jar = Jar)]
fn tracked_fn(db: &dyn Db, input: MyInput) -> u32 {
    input.field(db) * 2
}

#[test]
fn execute() {
    #[timed_salsa::db(Jar)]
    #[derive(Default)]
    struct Database {
        storage: timed_salsa::Storage<Self>,
    }

    impl timed_salsa::Database for Database {}

    impl Db for Database {}

    let mut db = Database::default();
    let input = MyInput::new(&db, 22);
    assert_eq!(tracked_fn(&db, input), 44);
}
