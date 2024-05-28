use super::*;
use husky_syn_decl::decl::ty_variant::unit_ty_variant::TypeUnitVariantSynDecl;

#[salsa::interned]
pub struct EnumUnitTypeVariantDecTemplate {
    pub parent_ty_template: EnumDecTemplate,
    pub self_ty: DecTerm,
}

impl EnumUnitTypeVariantDecTemplate {
    pub(super) fn from_decl(
        db: &::salsa::Db,
        parent_ty_template: EnumDecTemplate,
        decl: TypeUnitVariantSynDecl,
    ) -> DecSignatureResult<Self> {
        // todo: GADT
        Ok(EnumUnitTypeVariantDecTemplate::new(
            db,
            parent_ty_template,
            parent_ty_template.self_ty(db),
        ))
    }
}
