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
    todo!()
}

#[salsa::tracked(jar = HirDefnJar)]
fn extern_hir_defn_version_stamp(
    db: &dyn HirDefnDb,
    hir_defn: ExternHirDefn,
) -> HirDefnVersionStamp {
    todo!()
}
