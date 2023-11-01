pub trait RustTranspilationDb: salsa::DbWithJar<RustTranspilationJar> {}

impl<Db> RustTranspilationDb for Db where Db: salsa::DbWithJar<RustTranspilationJar> {}

#[salsa::jar(db = RustTranspilationDb)]
pub struct RustTranspilationJar();
