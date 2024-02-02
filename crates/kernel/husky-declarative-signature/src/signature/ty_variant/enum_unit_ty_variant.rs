use crate::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct EnumUnitTypeVariantDecTemplate {
    pub parent_ty_template: EnumTypeDecTemplate,
    pub self_ty: DeclarativeTerm,
}

impl EnumUnitTypeVariantDecTemplate {
    pub(super) fn from_decl(
        db: &::salsa::Db,
        parent_ty_template: EnumTypeDecTemplate,
        decl: TypeUnitVariantSynDecl,
    ) -> DeclarativeSignatureResult<Self> {
        // todo: GADT
        Ok(EnumUnitTypeVariantDecTemplate::new(
            db,
            parent_ty_template,
            parent_ty_template.self_ty(db),
        ))
    }
}
