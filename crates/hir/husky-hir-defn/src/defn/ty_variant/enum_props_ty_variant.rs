use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar)]
pub struct EnumPropsVariantHirDefn {
    pub path: TypeVariantPath,
    pub hir_decl: EnumPropsTypeVariantHirDecl,
}

#[salsa::tracked(jar = HirDefnJar)]
fn enum_props_variant_hir_defn_dependencies(
    db: &dyn HirDefnDb,
    hir_defn: EnumPropsVariantHirDefn,
) -> HirDefnDependencies {
    todo!()
}

#[salsa::tracked(jar = HirDefnJar)]
fn enum_props_variant_hir_defn_version_stamp(
    db: &dyn HirDefnDb,
    hir_defn: EnumPropsVariantHirDefn,
) -> HirDefnVersionStamp {
    todo!()
}
