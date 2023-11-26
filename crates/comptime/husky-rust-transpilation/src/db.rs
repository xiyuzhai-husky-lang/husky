use husky_hir_defn::db::HirDefnDb;
use husky_linkage::db::LinkageDb;

pub trait RustTranspilationDb:
    salsa::DbWithJar<RustTranspilationJar> + HirDefnDb + LinkageDb
{
}

impl RustTranspilationDb for Db where
    Db: salsa::DbWithJar<RustTranspilationJar> + HirDefnDb + LinkageDb
{
}

#[salsa::jar(db = RustTranspilationDb)]
pub struct RustTranspilationJar(
    crate::defn::module_defn_rust_transpilation,
    crate::package::rust_transpilation_packages,
    crate::linkages::package_linkages_transpilation,
    crate::manifest::package_rust_manifest,
);
