#[timed_salsa::jar(db = Db)]
struct Jar(InputWithBannedName1, InputWithBannedName2);

// Banned field name: `from`
#[timed_salsa::input]
struct InputWithBannedName1 {
    from: u32,
}

// Banned field name: `new`
#[timed_salsa::input]
struct InputWithBannedName2 {
    new: u32,
}

trait Db: timed_salsa::DbWithJar<Jar> {}

fn main() {}
