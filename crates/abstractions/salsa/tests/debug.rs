//! Test that `DeriveWithDb` is correctly derived.

use expect_test::expect;
use salsa::*;

#[salsa::jar]
struct Jar(MyInput, ComplexStruct);

#[salsa::input(db = Db)]
struct MyInput {
    field: u32,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct NotSalsa {
    field: String,
}

#[salsa::input(db = Db)]
struct ComplexStruct {
    my_input: MyInput,
    not_salsa: NotSalsa,
}

#[salsa::db(Jar)]
#[derive(Default)]
struct Database;

#[test]
fn input() {
    let db = Database::default();

    let input = MyInput::new(&db, 22, salsa::Durability::LOW);
    let not_salsa = NotSalsa {
        field: "it's salsa time".to_string(),
    };
    let complex_struct = ComplexStruct::new(&db, input, not_salsa, salsa::Durability::LOW);

    // default debug only includes identity fields
    let actual = format!("{:?}", complex_struct.debug(&db));
    let expected = expect![[
        r#"ComplexStruct { my_input: MyInput { field: 22 }, not_salsa: NotSalsa { field: "it's salsa time" } }"#
    ]];
    expected.assert_eq(&actual);

    // all fields
    let actual = format!("{:?}", complex_struct.debug(&db));
    let expected = expect![[
        r#"ComplexStruct { my_input: MyInput { field: 22 }, not_salsa: NotSalsa { field: "it's salsa time" } }"#
    ]];
    expected.assert_eq(&actual);
}
