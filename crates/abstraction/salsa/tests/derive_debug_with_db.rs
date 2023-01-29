#[salsa::derive_debug_with_db(jar = Jar)]
struct A(usize, usize);

trait Db: salsa::DbWithJar<Jar> {}

#[salsa::jar(db = Db)]
struct Jar();
