trait Db: salsa::DbWithJar<Jar> {}

#[salsa::jar(db = Db)]
struct Jar(Keywords);

#[salsa::interned(db = Db, jar = Jar)]
struct Keywords {}
