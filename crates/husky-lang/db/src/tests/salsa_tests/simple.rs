#[salsa::query_group(SimpleQueryStorage)]
trait SimpleQuery: salsa::Database {
    #[salsa::input]
    fn input(&self) -> i32;

    fn output(&self) -> i32;
}

fn output(this: &dyn SimpleQuery) -> i32 {
    this.input()
}

#[salsa::database(SimpleQueryStorage)]
#[derive(Default)]
struct SimpleDatabase {
    storage: salsa::Storage<SimpleDatabase>,
}
impl salsa::Database for SimpleDatabase {}

#[test]
pub fn simple1() {
    let mut db = SimpleDatabase::default();
    db.set_input(1);
    assert_eq!(db.output(), 1);
}

#[test]
pub fn simple2() {
    let mut db = SimpleDatabase::default();
    db.set_input(1);
    assert_eq!(db.output(), 1);
    db.set_input(2);
    assert_eq!(db.output(), 2);
}

#[test]
pub fn simple3() {
    let mut db = SimpleDatabase::default();
    db.set_input(1);
    assert_eq!(db.output(), 1);
    db.set_input(2);
    assert_eq!(db.output(), 2);
}
