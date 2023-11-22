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
    todo!()
}

#[salsa::tracked(jar = HirDefnJar)]
fn union_hir_defn_version_stamp(db: &dyn HirDefnDb, hir_defn: UnionHirDefn) -> HirDefnVersionStamp {
    todo!()
}
