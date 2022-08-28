#[cfg(test)]
use crate::*;

#[salsa::query_group(MyQueryStorage)]
trait MyQuery: salsa::Database {
    #[salsa::input]
    fn input(&self) -> i32;

    fn output(&self) -> i32;

    fn output2(&self) -> i32;
}

fn output(db: &dyn MyQuery) -> i32 {
    db.salsa_runtime()
        .report_synthetic_read(salsa::Durability::LOW);
    db.input()
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
}

impl salsa::Database for MyDatabase {}

#[test]
fn change_file() {
    let mut db = MyDatabase::default();
    db.set_input(0);
    should_eq!(db.output(), 0);
    should_eq!(db.output2(), 0);
    db.set_input(1);
    should_eq!(db.output(), 1);
    should_eq!(db.output2(), 1);
}
