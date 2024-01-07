use expect_test::expect;
use husky_salsa_log_utils::{HasLogger};
use salsa::{Db, DebugWithDb};

#[salsa::jar(db = Db)]
struct Jar(MyInput, MyTracked, tracked_fn, tracked_fn_extra);

#[salsa::input(db = Db, jar = Jar)]
struct MyInput {
    field: u32,
}

#[salsa::tracked(db = Db, jar = Jar)]
struct MyTracked {
    field: u32,
}

#[salsa::tracked(jar = Jar)]
fn tracked_fn(db: &Db, input: MyInput) -> u32 {
    db.push_log(format!("tracked_fn({:?})", input.debug(db)));
    let t = MyTracked::new(db, input.field(db) * 2);
    tracked_fn_extra::specify(db, t, 2222);
    tracked_fn_extra(db, t)
}

#[salsa::tracked(jar = Jar, specify)]
fn tracked_fn_extra(db: &Db, input: MyTracked) -> u32 {
    db.push_log(format!("tracked_fn_extra({:?})", input.debug(db)));
    0
}

#[salsa::db(Jar)]
struct Database;

#[test]
fn execute() {
    let mut db = Database::default();
    let input = MyInput::new(&db, 22);
    assert_eq!(tracked_fn(&db, input), 2222);
    db.assert_logs(expect![[r#"
        [
            "tracked_fn(MyInput { field: 22 })",
        ]"#]]);

    // A "synthetic write" causes the system to act *as though* some
    // input of durability `durability` has changed.
    db.synthetic_write(salsa::Durability::LOW);

    // Re-run the query on the original input. Nothing re-executes!
    assert_eq!(tracked_fn(&db, input), 2222);
    db.assert_logs(expect!["[]"]);
}
