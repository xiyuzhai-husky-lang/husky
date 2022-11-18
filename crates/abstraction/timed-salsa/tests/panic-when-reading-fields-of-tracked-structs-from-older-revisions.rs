use test_log::test;

#[timed_salsa::jar(db = Db)]
struct Jar(MyInput, MyTracked, tracked_fn);

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
    MyTracked::new(db, input.field(db) / 2)
}

#[timed_salsa::db(Jar)]
#[derive(Default)]
struct Database {
    storage: timed_salsa::Storage<Self>,
}

impl timed_salsa::Database for Database {}

impl Db for Database {}

#[test]
#[should_panic(expected = "`execute` method for field")]
fn execute() {
    let mut db = Database::default();

    let input = MyInput::new(&db, 22);
    let tracked = tracked_fn(&db, input);

    // modify the input and change the revision
    input.set_field(&mut db).to(24);

    // panic when reading fields of tracked structs from older revisions
    tracked.field(&db);
}
