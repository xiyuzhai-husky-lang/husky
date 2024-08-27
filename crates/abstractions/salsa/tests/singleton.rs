//! Basic Singleton struct test:
//!
//! Singleton structs are created only once. Subsequent `get`s and `new`s after creation return the same `Id`.

use expect_test::expect;

use salsa::DebugWithDb;

use test_log::test;

#[salsa::jar]
struct Jar(MyInput);

#[salsa::input(db = Db, singleton)]
struct MyInput {
    field: u32,
    #[id]
    id_field: u16,
}

#[salsa::db(Jar)]
struct Database;

#[test]
fn basic() {
    let db = Database::default();
    let input1 = MyInput::new(&db, 3, 4, salsa::Durability::LOW);
    let input2 = MyInput::get(&db);

    assert_eq!(input1, input2);

    let input3 = MyInput::try_get(&db);
    assert_eq!(Some(input1), input3);
}

#[test]
#[should_panic]
fn twice() {
    let db = Database::default();
    let input1 = MyInput::new(&db, 3, 4, salsa::Durability::LOW);
    let input2 = MyInput::get(&db);

    assert_eq!(input1, input2);

    // should panic here
    _ = MyInput::new(&db, 3, 5, salsa::Durability::LOW);
}

#[test]
fn debug() {
    let db = Database::default();
    let input = MyInput::new(&db, 3, 4, salsa::Durability::LOW);
    let actual = format!("{:?}", input.debug(&db));
    let expected = expect!["MyInput { field: 3, id_field: 4 }"];
    expected.assert_eq(&actual);
}
