#![allow(warnings)]

use expect_test::expect;
use salsa::DebugWithDb;

#[salsa::jar(db = Db)]
struct Jar(MyInput);

#[salsa::input(db = Db, jar = Jar)]
struct MyInput {
    #[id]
    id_one: u32,
    #[id]
    id_two: u16,
    field: u8,
}

#[salsa::db(Jar)]
struct Database;

#[test]
fn test_debug() {
    let mut db = Database::default();

    let input = MyInput::new(&mut db, 22, 50, 10);

    let actual = format!("{:?}", input.debug(&db));
    let expected = expect!["MyInput { [salsa id]: 0, id_one: 22, id_two: 50, field: 10 }"];
    expected.assert_eq(&actual);
}
