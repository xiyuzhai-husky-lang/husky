use super::*;
use husky_syn_decl::UnitTypeVariantSynDecl;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct EnumUnitTypeVariantHirDecl {
    pub path: TypeVariantPath,
    pub hir_eager_expr_region: HirEagerExprRegion,
}

impl EnumUnitTypeVariantHirDecl {
    pub(super) fn from_syn(
        path: TypeVariantPath,
        syn_decl: UnitTypeVariantSynDecl,
        db: &::salsa::Db,
    ) -> Self {
        let builder = HirDeclBuilder::new(syn_decl.syn_expr_region(db), db);
        Self::new(db, path, builder.finish().eager())
    }
}
