use std::sync::Mutex;

use common::*;

trait InputProvider {
    fn provide_input(&self) -> i32;
}

#[salsa::query_group(MyQueryStorage)]
trait MyQuery: salsa::Database + Invalidator + InputProvider {
    fn read(&self, key: ()) -> i32;

    fn output(&self) -> i32;

    fn output2(&self) -> i32;
}

trait Invalidator {
    fn invalidate_input(&mut self);
}

fn read(db: &dyn MyQuery, key: ()) -> i32 {
    db.salsa_runtime()
        .report_synthetic_read(salsa::Durability::LOW);
    db.provide_input()
}

fn output(db: &dyn MyQuery) -> i32 {
    db.salsa_runtime()
        .report_synthetic_read(salsa::Durability::LOW);
    db.read(())
}

fn output2(db: &dyn MyQuery) -> i32 {
    db.salsa_runtime()
        .report_synthetic_read(salsa::Durability::LOW);
    db.output()
}

#[salsa::database(MyQueryStorage)]
#[derive(Default)]
struct MyDatabase {
    storage: salsa::Storage<MyDatabase>,
    global_value: i32,
}

impl salsa::Database for MyDatabase {}

impl InputProvider for MyDatabase {
    fn provide_input(&self) -> i32 {
        self.global_value
    }
}

impl Invalidator for MyDatabase {
    fn invalidate_input(&mut self) {
        ReadQuery.in_db_mut(self).invalidate(&());
    }
}

#[test]
fn change_file() {
    let mut db = MyDatabase::default();
    assert_eq!(db.output(), 0);
    assert_eq!(db.output2(), 0);
    db.global_value = 1;
    assert_eq!(db.output(), 0);
    assert_eq!(db.output2(), 0);
    db.invalidate_input();
    assert_eq!(db.output(), 1);
    ep!(db.output2());
    assert_eq!(db.output2(), 1);
}
