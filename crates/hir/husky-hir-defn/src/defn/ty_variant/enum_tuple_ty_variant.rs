use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar)]
pub struct EnumTupleVariantHirDefn {
    pub path: TypeVariantPath,
    pub hir_decl: EnumTupleTypeVariantHirDecl,
}

impl EnumTupleVariantHirDefn {
    pub(super) fn dependencies(self, db: &dyn HirDefnDb) -> HirDefnDependencies {
        enum_tuple_variant_hir_defn_dependencies(db, self)
    }

    pub(super) fn version_stamp(self, db: &dyn HirDefnDb) -> HirDefnVersionStamp {
        enum_tuple_variant_hir_defn_version_stamp(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
fn enum_tuple_variant_hir_defn_dependencies(
    db: &dyn HirDefnDb,
    hir_defn: EnumTupleVariantHirDefn,
) -> HirDefnDependencies {
    todo!()
}

#[salsa::tracked(jar = HirDefnJar)]
fn enum_tuple_variant_hir_defn_version_stamp(
    db: &dyn HirDefnDb,
    hir_defn: EnumTupleVariantHirDefn,
) -> HirDefnVersionStamp {
    todo!()
}
