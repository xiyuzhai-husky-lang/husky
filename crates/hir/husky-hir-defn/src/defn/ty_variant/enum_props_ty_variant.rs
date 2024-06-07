use super::*;
use husky_hir_decl::decl::EnumPropsVariantHirDecl;

#[salsa::interned]
pub struct EnumPropsVariantHirDefn {
    pub path: TypeVariantPath,
    pub hir_decl: EnumPropsVariantHirDecl,
}

impl From<EnumPropsVariantHirDefn> for HirDefn {
    fn from(hir_defn: EnumPropsVariantHirDefn) -> Self {
        HirDefn::TypeVariant(hir_defn.into())
    }
}

impl EnumPropsVariantHirDefn {
    pub(super) fn deps(self, db: &::salsa::Db) -> HirDefnDeps {
        enum_props_variant_hir_defn_deps(db, self)
    }

    pub(super) fn version_stamp(self, db: &::salsa::Db) -> HirDefnVersionStamp {
        enum_props_variant_hir_defn_version_stamp(db, self)
    }
}

#[salsa::tracked]
fn enum_props_variant_hir_defn_deps(
    db: &::salsa::Db,
    hir_defn: EnumPropsVariantHirDefn,
) -> HirDefnDeps {
    let mut builder = HirDefnDepsBuilder::new(hir_defn.path(db), db);
    let hir_decl = hir_defn.hir_decl(db);
    builder.add_hir_eager_expr_region(hir_decl.hir_eager_expr_region(db));
    for field in hir_decl.fields(db) {
        builder.add_hir_ty(field.ty())
    }
    builder.finish()
}

#[salsa::tracked]
fn enum_props_variant_hir_defn_version_stamp(
    db: &::salsa::Db,
    hir_defn: EnumPropsVariantHirDefn,
) -> HirDefnVersionStamp {
    HirDefnVersionStamp::new(hir_defn, db)
}
