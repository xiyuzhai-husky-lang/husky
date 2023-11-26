use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar)]
pub struct ExternHirDefn {
    pub path: TypePath,
    pub hir_decl: ExternTypeHirDecl,
}

impl From<ExternHirDefn> for MajorItemHirDefn {
    fn from(hir_defn: ExternHirDefn) -> Self {
        MajorItemHirDefn::Type(hir_defn.into())
    }
}

impl From<ExternHirDefn> for HirDefn {
    fn from(hir_defn: ExternHirDefn) -> Self {
        HirDefn::MajorItem(hir_defn.into())
    }
}

impl ExternHirDefn {
    pub(super) fn dependencies(self, db: &::salsa::Db) -> HirDefnDependencies {
        extern_hir_defn_dependencies(db, self)
    }

    pub(super) fn version_stamp(self, db: &::salsa::Db) -> HirDefnVersionStamp {
        extern_hir_defn_version_stamp(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
fn extern_hir_defn_dependencies(db: &::salsa::Db, hir_defn: ExternHirDefn) -> HirDefnDependencies {
    let mut builder = HirDefnDependenciesBuilder::new(hir_defn.path(db), db);
    let hir_decl = hir_defn.hir_decl(db);
    builder.add_hir_eager_expr_region(hir_decl.hir_eager_expr_region(db));
    builder.finish()
}

#[salsa::tracked(jar = HirDefnJar)]
fn extern_hir_defn_version_stamp(db: &::salsa::Db, hir_defn: ExternHirDefn) -> HirDefnVersionStamp {
    HirDefnVersionStamp::new(hir_defn, db)
}
