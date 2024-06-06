use super::*;
use husky_hir_decl::decl::ExternTypeHirDecl;

#[salsa::interned]
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
    pub(super) fn deps(self, db: &::salsa::Db) -> HirDefnDeps {
        extern_hir_defn_deps(db, self)
    }

    pub(super) fn version_stamp(self, db: &::salsa::Db) -> HirDefnVersionStamp {
        extern_hir_defn_version_stamp(db, self)
    }
}

#[salsa::tracked]
fn extern_hir_defn_deps(db: &::salsa::Db, hir_defn: ExternHirDefn) -> HirDefnDeps {
    let mut builder = HirDefnDepsBuilder::new(hir_defn.path(db), db);
    let hir_decl = hir_defn.hir_decl(db);
    builder.add_hir_eager_expr_region(hir_decl.hir_eager_expr_region(db));
    builder.finish()
}

#[salsa::tracked]
fn extern_hir_defn_version_stamp(db: &::salsa::Db, hir_defn: ExternHirDefn) -> HirDefnVersionStamp {
    HirDefnVersionStamp::new(hir_defn, db)
}
