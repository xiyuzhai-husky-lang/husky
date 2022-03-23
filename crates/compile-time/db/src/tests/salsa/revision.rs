use crate::*;
use std::cell::Cell;

pub trait IssueUid {
    fn issue_uid(&self, x: i32) -> i32;
}

#[salsa::query_group(MyQueryStorage)]
trait MyQuery: IssueUid {
    #[salsa::input]
    fn x(&self) -> i32;

    // be careful, must use this intermediate y
    fn y(&self) -> i32;

    fn revision(&self) -> i32;
}

fn y(db: &dyn MyQuery) -> i32 {
    db.x()
}

fn revision(db: &dyn MyQuery) -> i32 {
    db.issue_uid(db.y())
}

#[salsa::database(MyQueryStorage)]
#[derive(Default)]
struct MyDatabase {
    storage: salsa::Storage<MyDatabase>,
    next_uid: Cell<i32>,
}

impl salsa::Database for MyDatabase {}

impl IssueUid for MyDatabase {
    fn issue_uid(&self, _: i32) -> i32 {
        let uid = self.next_uid.get();
        self.next_uid.set(uid + 1);
        uid
    }
}

#[test]
fn test_revisions() {
    let mut db = MyDatabase::default();
    db.set_x(0);
    should_eq!(db.revision(), 0);
    db.set_x(0);
    should_eq!(db.revision(), 0);
    db.set_x(2);
    should_eq!(db.revision(), 1);
    db.set_x(0);
    should_eq!(db.revision(), 2);
}
