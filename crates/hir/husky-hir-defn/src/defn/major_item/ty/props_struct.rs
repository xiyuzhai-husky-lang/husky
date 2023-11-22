use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar)]
pub struct PropsStructHirDefn {
    pub path: TypePath,
    pub hir_decl: PropsStructTypeHirDecl,
}

impl PropsStructHirDefn {
    pub(super) fn dependencies(self, db: &dyn HirDefnDb) -> HirDefnDependencies {
        props_struct_hir_defn_dependencies(db, self)
    }

    pub(super) fn version_stamp(self, db: &dyn HirDefnDb) -> HirDefnVersionStamp {
        props_struct_hir_defn_version_stamp(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
fn props_struct_hir_defn_dependencies(
    db: &dyn HirDefnDb,
    hir_defn: PropsStructHirDefn,
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
fn props_struct_hir_defn_version_stamp(
    db: &dyn HirDefnDb,
    hir_defn: PropsStructHirDefn,
) -> HirDefnVersionStamp {
    todo!()
}
