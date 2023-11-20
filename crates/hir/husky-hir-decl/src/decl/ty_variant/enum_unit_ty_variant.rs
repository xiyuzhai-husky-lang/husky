use super::*;
use husky_syn_decl::UnitTypeVariantSynDecl;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct EnumUnitTypeVariantHirDecl {
    pub path: TypeVariantPath,
}

impl EnumUnitTypeVariantHirDecl {
    pub(super) fn from_syn(
        path: TypeVariantPath,
        _syn_decl: UnitTypeVariantSynDecl,
        db: &dyn HirDeclDb,
    ) -> Self {
        Self::new(db, path)
    }
}
