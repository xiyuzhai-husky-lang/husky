//! Test that a constant `tracked` fn (has no inputs)
//! compiles and executes successfully.
#![allow(warnings)]

#[timed_salsa::jar(db = Db)]
struct Jar(tracked_fn);

trait Db: timed_salsa::DbWithJar<Jar> {}

#[timed_salsa::tracked]
fn tracked_fn(db: &dyn Db) -> u32 {
    44
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
    assert_eq!(tracked_fn(&db), 44);
}
