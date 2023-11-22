use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar)]
pub struct UnionHirDefn {
    pub path: TypePath,
    pub hir_decl: UnionHirDecl,
}

impl UnionHirDefn {
    pub(super) fn dependencies(self, db: &dyn HirDefnDb) -> HirDefnDependencies {
        union_hir_defn_dependencies(db, self)
    }

    pub(super) fn version_stamp(self, db: &dyn HirDefnDb) -> HirDefnVersionStamp {
        union_hir_defn_version_stamp(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
fn union_hir_defn_dependencies(db: &dyn HirDefnDb, hir_defn: UnionHirDefn) -> HirDefnDependencies {
    let mut builder = HirDefnDependenciesBuilder::new(hir_defn.path(db), db);
    let hir_decl = hir_defn.hir_decl(db);
    builder.add_hir_eager_expr_region(hir_decl.hir_eager_expr_region(db));
    builder.finish()
}

#[salsa::tracked(jar = HirDefnJar)]
fn union_hir_defn_version_stamp(db: &dyn HirDefnDb, hir_defn: UnionHirDefn) -> HirDefnVersionStamp {
    todo!()
}
