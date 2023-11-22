use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar)]
pub struct ExternHirDefn {
    pub path: TypePath,
    pub hir_decl: ExternTypeHirDecl,
}

impl ExternHirDefn {
    pub(super) fn dependencies(self, db: &dyn HirDefnDb) -> HirDefnDependencies {
        extern_hir_defn_dependencies(db, self)
    }

    pub(super) fn version_stamp(self, db: &dyn HirDefnDb) -> HirDefnVersionStamp {
        extern_hir_defn_version_stamp(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
fn extern_hir_defn_dependencies(
    db: &dyn HirDefnDb,
    hir_defn: ExternHirDefn,
) -> HirDefnDependencies {
    let mut builder = HirDefnDependenciesBuilder::new(hir_defn.path(db), db);
    let hir_decl = hir_defn.hir_decl(db);
    builder.add_hir_eager_expr_region(hir_decl.hir_eager_expr_region(db));
    builder.finish()
}

#[salsa::tracked(jar = HirDefnJar)]
fn extern_hir_defn_version_stamp(
    db: &dyn HirDefnDb,
    hir_defn: ExternHirDefn,
) -> HirDefnVersionStamp {
    todo!()
}
