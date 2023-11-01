use husky_hir_defn::db::HirDefnDb;

pub trait RustTranspilationDb: salsa::DbWithJar<RustTranspilationJar> + HirDefnDb {}

impl<Db> RustTranspilationDb for Db where Db: salsa::DbWithJar<RustTranspilationJar> + HirDefnDb {}

#[salsa::jar(db = RustTranspilationDb)]
pub struct RustTranspilationJar(crate::defn::module_defn_rust_transpilation);
