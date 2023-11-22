use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar)]
pub struct UnitStructHirDefn {
    pub path: TypePath,
    pub hir_decl: UnitStructHirDecl,
}

impl UnitStructHirDefn {
    pub(super) fn dependencies(self, db: &dyn HirDefnDb) -> HirDefnDependencies {
        unit_struct_hir_defn_dependencies(db, self)
    }

    pub(super) fn version_stamp(self, db: &dyn HirDefnDb) -> HirDefnVersionStamp {
        unit_struct_hir_defn_version_stamp(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
fn unit_struct_hir_defn_dependencies(
    db: &dyn HirDefnDb,
    hir_defn: UnitStructHirDefn,
) -> HirDefnDependencies {
    todo!()
}

#[salsa::tracked(jar = HirDefnJar)]
fn unit_struct_hir_defn_version_stamp(
    db: &dyn HirDefnDb,
    hir_defn: UnitStructHirDefn,
) -> HirDefnVersionStamp {
    todo!()
}
