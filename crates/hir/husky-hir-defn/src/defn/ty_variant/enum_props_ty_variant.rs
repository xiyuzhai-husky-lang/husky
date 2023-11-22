use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar)]
pub struct EnumPropsVariantHirDefn {
    pub path: TypeVariantPath,
    pub hir_decl: EnumPropsVariantHirDecl,
}

#[salsa::tracked(jar = HirDefnJar)]
fn enum_props_variant_hir_defn_dependencies(
    db: &dyn HirDefnDb,
    hir_defn: EnumPropsVariantHirDefn,
) -> HirDefnDependencies {
    let mut builder = HirDefnDependenciesBuilder::new(hir_defn.path(db), db);
    let hir_decl = hir_defn.hir_decl(db);
    builder.add_hir_eager_expr_region(hir_decl.hir_eager_expr_region(db));
    for field in hir_decl.fields(db) {
        builder.add_hir_ty(field.ty())
    }
    builder.finish()
}

#[salsa::tracked(jar = HirDefnJar)]
fn enum_props_variant_hir_defn_version_stamp(
    db: &dyn HirDefnDb,
    hir_defn: EnumPropsVariantHirDefn,
) -> HirDefnVersionStamp {
    todo!()
}
