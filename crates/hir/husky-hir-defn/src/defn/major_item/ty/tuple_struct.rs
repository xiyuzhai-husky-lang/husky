use super::*;
use husky_hir_decl::decl::TupleStructHirDecl;

#[salsa::interned]
pub struct TupleStructHirDefn {
    pub path: TypePath,
    pub hir_decl: TupleStructHirDecl,
}

impl From<TupleStructHirDefn> for MajorItemHirDefn {
    fn from(hir_defn: TupleStructHirDefn) -> Self {
        MajorItemHirDefn::Type(hir_defn.into())
    }
}

impl From<TupleStructHirDefn> for HirDefn {
    fn from(hir_defn: TupleStructHirDefn) -> Self {
        HirDefn::MajorItem(hir_defn.into())
    }
}

impl TupleStructHirDefn {
    pub(super) fn deps(self, db: &::salsa::Db) -> HirDefnDeps {
        tuple_struct_hir_defn_deps(db, self)
    }

    pub(super) fn version_stamp(self, db: &::salsa::Db) -> HirDefnVersionStamp {
        tuple_struct_hir_defn_version_stamp(db, self)
    }
}

#[salsa::tracked]
fn tuple_struct_hir_defn_deps(db: &::salsa::Db, hir_defn: TupleStructHirDefn) -> HirDefnDeps {
    let mut builder = HirDefnDepsBuilder::new(hir_defn.path(db), db);
    let hir_decl = hir_defn.hir_decl(db);
    builder.add_hir_eager_expr_region(hir_decl.hir_eager_expr_region(db));
    for field in hir_decl.fields(db) {
        builder.add_hir_ty(field.ty())
    }
    builder.finish()
}

#[salsa::tracked]
fn tuple_struct_hir_defn_version_stamp(
    db: &::salsa::Db,
    hir_defn: TupleStructHirDefn,
) -> HirDefnVersionStamp {
    HirDefnVersionStamp::new(hir_defn, db)
}
