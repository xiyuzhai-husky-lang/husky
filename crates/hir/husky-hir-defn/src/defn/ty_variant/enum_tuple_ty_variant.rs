use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar)]
pub struct EnumTupleVariantHirDefn {
    pub path: TypeVariantPath,
    pub hir_decl: EnumTupleVariantHirDecl,
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
