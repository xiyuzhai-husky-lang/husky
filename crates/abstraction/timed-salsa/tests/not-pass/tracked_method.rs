//! Test that a `tracked` fn on a `timed_salsa::input`
//! compiles and executes successfully.
#![allow(warnings)]

#[timed_salsa::jar(db = Db)]
struct Jar(
    MyInput,
    MyInput_tracked_fn,
    MyInput_tracked_fn_ref,
    MyInput_TrackedTrait_tracked_trait_fn,
);

trait Db: timed_salsa::DbWithJar<Jar> {}

trait TrackedTrait {
    fn tracked_trait_fn(self, db: &dyn Db) -> u32;
}

#[timed_salsa::input]
struct MyInput {
    field: u32,
}

#[timed_salsa::tracked]
impl MyInput {
    #[timed_salsa::tracked]
    fn tracked_fn(self, db: &dyn Db) -> u32 {
        self.field(db) * 2
    }

    #[timed_salsa::tracked(return_ref)]
    fn tracked_fn_ref(self, db: &dyn Db) -> u32 {
        self.field(db) * 3
    }
}

#[timed_salsa::tracked]
impl TrackedTrait for MyInput {
    #[timed_salsa::tracked]
    fn tracked_trait_fn(self, db: &dyn Db) -> u32 {
        self.field(db) * 4
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
    let object = MyInput::new(&mut db, 22);
    assert_eq!(object.tracked_fn(&db), 44);
    assert_eq!(*object.tracked_fn_ref(&db), 66);
    assert_eq!(object.tracked_trait_fn(&db), 88);
}
