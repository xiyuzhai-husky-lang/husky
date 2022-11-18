use expect_test::expect;
use husky_salsa_log_utils::{HasLogger, Logger};
use timed_salsa::{Database as SalsaDatabase, DebugWithDb};

#[timed_salsa::jar(db = Db)]
struct Jar(MyInput, MyTracked, tracked_fn, tracked_fn_extra);

trait Db: timed_salsa::DbWithJar<Jar> + HasLogger {}

#[timed_salsa::input(jar = Jar)]
struct MyInput {
    field: u32,
}

#[timed_salsa::tracked(jar = Jar)]
struct MyTracked {
    field: u32,
}

#[timed_salsa::tracked(jar = Jar)]
fn tracked_fn(db: &dyn Db, input: MyInput) -> u32 {
    db.push_log(format!("tracked_fn({:?})", input.debug(db)));
    let t = MyTracked::new(db, input.field(db) * 2);
    tracked_fn_extra::specify(db, t, 2222);
    tracked_fn_extra(db, t)
}

#[timed_salsa::tracked(jar = Jar, specify)]
fn tracked_fn_extra(db: &dyn Db, input: MyTracked) -> u32 {
    db.push_log(format!("tracked_fn_extra({:?})", input.debug(db)));
    0
}

#[timed_salsa::db(Jar)]
#[derive(Default)]
struct Database {
    storage: timed_salsa::Storage<Self>,
    logger: Logger,
}

impl timed_salsa::Database for Database {}

impl Db for Database {}

impl HasLogger for Database {
    fn logger(&self) -> &Logger {
        &self.logger
    }
}

#[test]
fn execute() {
    let mut db = Database::default();
    let input = MyInput::new(&db, 22);
    assert_eq!(tracked_fn(&db, input), 2222);
    db.assert_logs(expect![[r#"
        [
            "tracked_fn(MyInput { [salsa id]: 0 })",
        ]"#]]);

    // A "synthetic write" causes the system to act *as though* some
    // input of durability `durability` has changed.
    db.synthetic_write(timed_salsa::Durability::LOW);

    // Re-run the query on the original input. Nothing re-executes!
    assert_eq!(tracked_fn(&db, input), 2222);
    db.assert_logs(expect!["[]"]);
}
