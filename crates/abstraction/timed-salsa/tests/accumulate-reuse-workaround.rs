//! Demonstrates the workaround of wrapping calls to
//! `accumulated` in a tracked function to get better
//! reuse.

use husky_salsa_log_utils::{HasLogger, Logger};

use expect_test::expect;
use test_log::test;

#[timed_salsa::jar(db = Db)]
struct Jar(List, Integers, compute, accumulated);

trait Db: timed_salsa::DbWithJar<Jar> + HasLogger {}

#[timed_salsa::input]
struct List {
    value: u32,
    next: Option<List>,
}

#[timed_salsa::accumulator]
struct Integers(u32);

#[timed_salsa::tracked]
fn compute(db: &dyn Db, input: List) -> u32 {
    db.push_log(format!("compute({:?})", input,));

    // always pushes 0
    Integers::push(db, 0);

    let result = if let Some(next) = input.next(db) {
        let next_integers = accumulated(db, next);
        let v = input.value(db) + next_integers.iter().sum::<u32>();
        v
    } else {
        input.value(db)
    };

    // return value changes
    result
}

#[timed_salsa::tracked(return_ref)]
fn accumulated(db: &dyn Db, input: List) -> Vec<u32> {
    db.push_log(format!("accumulated({:?})", input,));
    compute::accumulated::<Integers>(db, input)
}

#[timed_salsa::db(Jar)]
#[derive(Default)]
struct Database {
    storage: timed_salsa::Storage<Self>,
    logger: Logger,
}

impl timed_salsa::Database for Database {
    fn salsa_event(&self, _event: timed_salsa::Event) {}
}

impl Db for Database {}

impl HasLogger for Database {
    fn logger(&self) -> &Logger {
        &self.logger
    }
}

#[test]
fn test1() {
    let mut db = Database::default();

    let l1 = List::new(&db, 1, None);
    let l2 = List::new(&db, 2, Some(l1));

    assert_eq!(compute(&db, l2), 2);
    db.assert_logs(expect![[r#"
        [
            "compute(List(Id { value: 2 }))",
            "accumulated(List(Id { value: 1 }))",
            "compute(List(Id { value: 1 }))",
        ]"#]]);

    // When we mutate `l1`, we should re-execute `compute` for `l1`,
    // and we re-execute accumulated for `l1`, but we do NOT re-execute
    // `compute` for `l2`.
    l1.set_value(&mut db).to(2);
    assert_eq!(compute(&db, l2), 2);
    db.assert_logs(expect![[r#"
        [
            "accumulated(List(Id { value: 1 }))",
            "compute(List(Id { value: 1 }))",
        ]"#]]);
}
