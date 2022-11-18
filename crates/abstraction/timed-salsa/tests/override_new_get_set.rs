//! Test that the `constructor` macro overrides
//! the `new` method's name and `get` and `set`
//! change the name of the getter and setter of the fields.
#![allow(warnings)]

use std::fmt::Display;

#[timed_salsa::jar(db = Db)]
struct Jar(MyInput, MyInterned, MyTracked);

trait Db: timed_salsa::DbWithJar<Jar> {}

#[timed_salsa::input(jar = Jar, constructor = from_string)]
struct MyInput {
    #[get(text)]
    #[set(set_text)]
    field: String,
}

impl MyInput {
    pub fn new(db: &mut dyn Db, s: impl Display) -> MyInput {
        MyInput::from_string(db, s.to_string())
    }

    pub fn field(self, db: &dyn Db) -> String {
        self.text(db)
    }

    pub fn set_field(self, db: &mut dyn Db, id: String) {
        self.set_text(db).to(id);
    }
}

#[timed_salsa::interned(constructor = from_string)]
struct MyInterned {
    #[get(text)]
    #[return_ref]
    field: String,
}

impl MyInterned {
    pub fn new(db: &dyn Db, s: impl Display) -> MyInterned {
        MyInterned::from_string(db, s.to_string())
    }

    pub fn field(self, db: &dyn Db) -> &str {
        &self.text(db)
    }
}

#[timed_salsa::tracked(constructor = from_string)]
struct MyTracked {
    #[get(text)]
    field: String,
}

impl MyTracked {
    pub fn new(db: &dyn Db, s: impl Display) -> MyTracked {
        MyTracked::from_string(db, s.to_string())
    }

    pub fn field(self, db: &dyn Db) -> String {
        self.text(db)
    }
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
}
