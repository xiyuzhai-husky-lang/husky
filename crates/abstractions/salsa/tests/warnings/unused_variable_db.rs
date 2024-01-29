#[salsa::jar]
struct Jar(Keywords);

#[salsa::interned(db = Db, jar = Jar)]
struct Keywords {}
