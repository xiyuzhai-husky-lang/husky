use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar, constructor = pub(super) new)]
pub struct EnumHirDefn {
    pub path: TypePath,
    pub hir_decl: EnumTypeHirDecl,
}

impl EnumHirDefn {
    pub(super) fn dependencies(self, db: &dyn HirDefnDb) -> HirDefnDependencies {
        enum_hir_defn_dependencies(db, self)
    }

    pub(super) fn version_stamp(self, db: &dyn HirDefnDb) -> HirDefnVersionStamp {
        enum_hir_defn_version_stamp(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
fn enum_hir_defn_dependencies(db: &dyn HirDefnDb, hir_defn: EnumHirDefn) -> HirDefnDependencies {
    todo!()
}

#[salsa::tracked(jar = HirDefnJar)]
fn enum_hir_defn_version_stamp(db: &dyn HirDefnDb, hir_defn: EnumHirDefn) -> HirDefnVersionStamp {
    todo!()
}
