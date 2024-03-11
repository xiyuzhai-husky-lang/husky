use crate::*;

#[salsa::interned(db = DecSignatureDb, jar = DecSignatureJar)]
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
