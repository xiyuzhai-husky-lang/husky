//! Test that `specify` does not work if the key is a `timed_salsa::interned`
//! compilation fails
#![allow(warnings)]

#[timed_salsa::jar(db = Db)]
struct Jar(MyInterned, MyTracked, tracked_fn);

trait Db: timed_salsa::DbWithJar<Jar> {}

#[timed_salsa::interned(jar = Jar)]
struct MyInterned {
    field: u32,
}

#[timed_salsa::tracked(jar = Jar)]
struct MyTracked {
    field: u32,
}

#[timed_salsa::tracked(jar = Jar, specify)]
fn tracked_fn(db: &dyn Db, input: MyInterned) -> MyTracked {
    MyTracked::new(db, input.field(db) * 2)
}

#[timed_salsa::db(Jar)]
#[derive(Default)]
struct Database {
    storage: timed_salsa::Storage<Self>,
}

impl timed_salsa::Database for Database {}

impl Db for Database {}

fn main() {}
