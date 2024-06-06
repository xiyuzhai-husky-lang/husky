use super::*;
use husky_hir_decl::decl::EnumUnitTypeVariantHirDecl;

#[salsa::interned]
pub struct EnumUnitVariantHirDefn {
    pub path: TypeVariantPath,
    pub hir_decl: EnumUnitTypeVariantHirDecl,
}

impl From<EnumUnitVariantHirDefn> for HirDefn {
    fn from(hir_defn: EnumUnitVariantHirDefn) -> Self {
        HirDefn::TypeVariant(hir_defn.into())
    }
}

impl EnumUnitVariantHirDefn {
    pub(super) fn deps(self, db: &::salsa::Db) -> HirDefnDeps {
        enum_unit_variant_hir_defn_deps(db, self)
    }

    pub(super) fn version_stamp(self, db: &::salsa::Db) -> HirDefnVersionStamp {
        enum_unit_variant_hir_defn_version_stamp(db, self)
    }
}

#[salsa::tracked]
fn enum_unit_variant_hir_defn_deps(
    db: &::salsa::Db,
    hir_defn: EnumUnitVariantHirDefn,
) -> HirDefnDeps {
    let builder = HirDefnDepsBuilder::new(hir_defn.path(db), db);
    let _hir_decl = hir_defn.hir_decl(db);
    // builder.add_hir_eager_expr_region(hir_decl.hir_eager_expr_region(db));
    builder.finish()
}

#[salsa::tracked]
fn enum_unit_variant_hir_defn_version_stamp(
    db: &::salsa::Db,
    hir_defn: EnumUnitVariantHirDefn,
) -> HirDefnVersionStamp {
    HirDefnVersionStamp::new(hir_defn, db)
}
