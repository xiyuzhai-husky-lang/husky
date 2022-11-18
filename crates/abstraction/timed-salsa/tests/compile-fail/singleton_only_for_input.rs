//! Compile Singleton struct test:
//!
//! Singleton flags are only allowed for input structs. If applied on any other Salsa struct compilation must fail

use husky_salsa_log_utils::{HasLogger, Logger};

use test_log::test;

#[timed_salsa::jar(db = Db)]
struct Jar(MyInput, MyTracked, Integers, create_tracked_structs);

trait Db: timed_salsa::DbWithJar<Jar> + HasLogger {}

#[timed_salsa::input(singleton)]
struct MyInput {
    field: u32,
}

#[timed_salsa::tracked(singleton)]
struct MyTracked {
    field: u32,
}

#[timed_salsa::tracked(singleton)]
fn create_tracked_structs(db: &dyn Db, input: MyInput) -> Vec<MyTracked> {
    (0..input.field(db))
        .map(|i| MyTracked::new(db, i))
        .collect()
}

#[timed_salsa::accumulator(singleton)]
struct Integers(u32);

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

fn main() {}
