trait Db: timed_salsa::DbWithJar<Jar> {}

#[timed_salsa::jar(db = Db)]
struct Jar(Keywords);

#[timed_salsa::interned(jar = Jar)]
struct Keywords {}
