//! Compile Singleton struct test:
//!
//! Singleton flags are only allowed for input structs. If applied on any other Salsa struct compilation must fail

use husky_salsa_log_utils::{HasLogger, Logger};

use test_log::test;

#[salsa::jar]
struct Jar(MyInput, MyTracked, Integers, create_tracked_structs);

#[salsa::input(singleton)]
struct MyInput {
    field: u32,
}

#[salsa::tracked(singleton)]
struct MyTracked {
    field: u32,
}

#[salsa::tracked(singleton)]
fn create_tracked_structs(db: &Db, input: MyInput) -> Vec<MyTracked> {
    (0..input.field(db))
        .map(|i| MyTracked::new(db, i))
        .collect()
}

#[salsa::accumulator(singleton)]
struct Integers(u32);

#[salsa::db(Jar)]
#[derive(Default)]
struct Database {
    storage: salsa::Storage<Self>,
    logger: Logger,
}

impl salsa::Database for Database {}

impl Db for Database {}

impl HasLogger for Database {
    fn logger(&self) -> &Logger {
        &self.logger
    }
}

fn main() {}
