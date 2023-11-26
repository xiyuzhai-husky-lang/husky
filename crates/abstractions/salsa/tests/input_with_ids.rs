#![allow(warnings)]

use expect_test::expect;
use salsa::DebugWithDb;

#[salsa::jar(db = Db)]
struct Jar(MyInput);

trait Db: salsa::DbWithJar<Jar> {}

impl<DB> Db for DB where DB: salsa::DbWithJar<Jar> {}

#[salsa::input(db = Db, jar = Jar)]
struct MyInput {
    #[id]
    id_one: u32,
    #[id]
    id_two: u16,
    field: u8,
}

#[salsa::db(Jar)]
#[derive(Default)]
struct Database {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for Database {}

#[test]
fn test_debug() {
    let mut db = Database::default();

    let input = MyInput::new(&mut db, 22, 50, 10);

    let actual = format!("{:?}", input.debug(&db));
    let expected = expect!["MyInput { [salsa id]: 0, field: 22, id_one: 50, id_two: 10 }"];
    expected.assert_eq(&actual);
}
