#[timed_salsa::jar(db = Db)]
struct Jar(MyInput);

trait Db: timed_salsa::DbWithJar<Jar> {}

#[timed_salsa::input(jar = Jar)]
struct MyInput {
    field: u32,
    #[id]
    id_one: u32,
}

#[timed_salsa::db(Jar)]
#[derive(Default)]
struct Database {
    storage: timed_salsa::Storage<Self>,
}

impl timed_salsa::Database for Database {}

impl Db for Database {}

fn main() {
    let mut db = Database::default();
    let input = MyInput::new(&mut db, 3, 4);
    // should not compile as `id_one` should not have a setter
    // compile error: method `set_id_one` not found in scope
    input.set_id_one(1);
}
