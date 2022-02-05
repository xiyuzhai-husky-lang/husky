use common::*;

pub trait IssueUid {
    fn issue_uid(&self, x: i32) -> i32;
}

#[salsa::query_group(MyQueryStorage)]
trait MyQuery: IssueUid {
    #[salsa::input]
    fn x(&self) -> i32;

    // be careful, must use this intermediate y
    fn y(&self) -> i32;

    fn uid(&self) -> i32;
}

fn y(db: &dyn MyQuery) -> i32 {
    db.x()
}

fn uid(db: &dyn MyQuery) -> i32 {
    db.issue_uid(db.y())
}

#[salsa::database(MyQueryStorage)]
#[derive(Default)]
struct MyDatabase {
    storage: salsa::Storage<MyDatabase>,
    last_uid: Cell<i32>,
}

impl salsa::Database for MyDatabase {}

impl IssueUid for MyDatabase {
    fn issue_uid(&self, _: i32) -> i32 {
        self.last_uid.set(self.last_uid.get() + 1);
        self.last_uid.get()
    }
}

#[test]
fn test_versions() {
    let mut db = MyDatabase::default();
    db.set_x(0);
    should_eq!(db.uid(), 1);
    db.set_x(0);
    should_eq!(db.uid(), 1);
    db.set_x(2);
    should_eq!(db.uid(), 2);
    db.set_x(0);
    should_eq!(db.uid(), 3);
}
