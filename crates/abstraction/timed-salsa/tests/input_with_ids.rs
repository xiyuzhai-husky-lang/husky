#![allow(warnings)]

use expect_test::expect;
use timed_salsa::DebugWithDb;

#[timed_salsa::jar(db = Db)]
struct Jar(MyInput);

trait Db: timed_salsa::DbWithJar<Jar> {}

#[timed_salsa::input(jar = Jar)]
struct MyInput {
    field: u32,
    #[id]
    id_one: u32,
    #[id]
    id_two: u16,
}

#[timed_salsa::db(Jar)]
#[derive(Default)]
struct Database {
    storage: timed_salsa::Storage<Self>,
}

impl timed_salsa::Database for Database {}

impl Db for Database {}

#[test]
fn test_debug() {
    let mut db = Database::default();

    let input = MyInput::new(&mut db, 22, 50, 10);

    let actual = format!("{:?}", input.debug(&db));
    let expected = expect![[r#"MyInput { [salsa id]: 0, id_one: 50, id_two: 10 }"#]];
    expected.assert_eq(&actual);
}
