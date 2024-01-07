//! Delete cascade:
//!
//! * when we delete memoized data, also delete outputs from that data

use husky_salsa_log_utils::{HasLogger};
use salsa::{Db};

use expect_test::expect;
use test_log::test;

#[salsa::jar(db = Db)]
struct Jar(
    MyInput,
    MyTracked,
    final_result,
    create_tracked_structs,
    contribution_from_struct,
    copy_field,
);

#[salsa::input(singleton)]
struct MyInput {
    field: u32,
}

#[salsa::tracked]
fn final_result(db: &Db, input: MyInput) -> u32 {
    db.push_log(format!("final_result({:?})", input));
    let mut sum = 0;
    for tracked_struct in create_tracked_structs(db, input) {
        sum += contribution_from_struct(db, tracked_struct);
    }
    sum
}

#[salsa::tracked(db = Db)]
struct MyTracked {
    field: u32,
}

#[salsa::tracked]
fn create_tracked_structs(db: &Db, input: MyInput) -> Vec<MyTracked> {
    db.push_log(format!("intermediate_result({:?})", input));
    (0..input.field(db))
        .map(|i| MyTracked::new(db, i))
        .collect()
}

#[salsa::tracked]
fn contribution_from_struct(db: &Db, tracked: MyTracked) -> u32 {
    let m = MyTracked::new(db, tracked.field(db));
    copy_field(db, m) * 2
}

#[salsa::tracked]
fn copy_field(db: &Db, tracked: MyTracked) -> u32 {
    tracked.field(db)
}

#[salsa::db(Jar)]
#[derive(Default)]
struct Database;

#[test]
fn basic() {
    let mut db = Database::default();

    // Creates 3 tracked structs
    let input = MyInput::new(&db, 3);
    assert_eq!(final_result(&db, input), 2 * 2 + 2);
    db.assert_logs(expect![[r#"
        [
            "final_result(MyInput(Id { value: 1 }))",
            "intermediate_result(MyInput(Id { value: 1 }))",
        ]"#]]);

    // Creates only 2 tracked structs in this revision, should delete 1
    //
    // Expect to see 6 DidDiscard events. Three from the primary struct:
    //
    // * the struct itself
    // * the struct's field
    // * the `contribution_from_struct` result
    //
    // and then 3 more from the struct created by `contribution_from_struct`:
    //
    // * the struct itself
    // * the struct's field
    // * the `copy_field` result

    input.set_field(&mut db).to(2);
    assert_eq!(final_result(&db, input), 2);
    db.assert_logs(expect![[r#"
        [
            "intermediate_result(MyInput(Id { value: 1 }))",
            "final_result(MyInput(Id { value: 1 }))",
        ]"#]]);
}
