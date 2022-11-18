#[timed_salsa::jar(db = Db)]
struct Jar(Tracked);

#[timed_salsa::tracked(jar = Jar)]
struct Tracked {
    field: u32,
}

impl Tracked {
    #[timed_salsa::tracked]
    fn use_tracked(&self) {}
}

trait Db: timed_salsa::DbWithJar<Jar> {}

fn main() {}
