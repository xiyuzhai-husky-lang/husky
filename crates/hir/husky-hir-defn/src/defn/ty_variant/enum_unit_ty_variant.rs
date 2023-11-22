use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar)]
pub struct EnumUnitVariantHirDefn {
    pub path: TypeVariantPath,
    pub hir_decl: EnumUnitTypeVariantHirDecl,
}

impl EnumUnitVariantHirDefn {
    pub(super) fn dependencies(self, db: &dyn HirDefnDb) -> HirDefnDependencies {
        enum_unit_variant_hir_defn_dependencies(db, self)
    }

    pub(super) fn version_stamp(self, db: &dyn HirDefnDb) -> HirDefnVersionStamp {
        enum_unit_variant_hir_defn_version_stamp(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
fn enum_unit_variant_hir_defn_dependencies(
    db: &dyn HirDefnDb,
    hir_defn: EnumUnitVariantHirDefn,
) -> HirDefnDependencies {
    todo!()
}

#[salsa::tracked(jar = HirDefnJar)]
fn enum_unit_variant_hir_defn_version_stamp(
    db: &dyn HirDefnDb,
    hir_defn: EnumUnitVariantHirDefn,
) -> HirDefnVersionStamp {
    todo!()
}
