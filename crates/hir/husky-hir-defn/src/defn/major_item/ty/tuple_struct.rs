use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar)]
pub struct TupleStructHirDefn {
    pub path: TypePath,
    pub hir_decl: TupleStructTypeHirDecl,
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
    pub(super) fn dependencies(self, db: &dyn HirDefnDb) -> HirDefnDependencies {
        tuple_struct_hir_defn_dependencies(db, self)
    }

    pub(super) fn version_stamp(self, db: &dyn HirDefnDb) -> HirDefnVersionStamp {
        tuple_struct_hir_defn_version_stamp(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
fn tuple_struct_hir_defn_dependencies(
    db: &dyn HirDefnDb,
    hir_defn: TupleStructHirDefn,
) -> HirDefnDependencies {
    let mut builder = HirDefnDependenciesBuilder::new(hir_defn.path(db), db);
    let hir_decl = hir_defn.hir_decl(db);
    builder.add_hir_eager_expr_region(hir_decl.hir_eager_expr_region(db));
    for field in hir_decl.fields(db) {
        builder.add_hir_ty(field.ty())
    }
    builder.finish()
}

#[salsa::tracked(jar = HirDefnJar)]
fn tuple_struct_hir_defn_version_stamp(
    db: &dyn HirDefnDb,
    hir_defn: TupleStructHirDefn,
) -> HirDefnVersionStamp {
    HirDefnVersionStamp::new(hir_defn, db)
}
