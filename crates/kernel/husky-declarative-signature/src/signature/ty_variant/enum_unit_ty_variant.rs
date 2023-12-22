use crate::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct EnumUnitTypeVariantDeclarativeSignatureTemplate {
    pub parent_ty_template: EnumTypeDeclarativeSignatureTemplate,
    pub self_ty: DeclarativeTerm,
}

impl EnumUnitTypeVariantDeclarativeSignatureTemplate {
    pub(super) fn from_decl(
        db: &::salsa::Db,
        parent_ty_template: EnumTypeDeclarativeSignatureTemplate,
        decl: UnitTypeVariantSynDecl,
    ) -> DeclarativeSignatureResult<Self> {
        // todo: GADT
        Ok(EnumUnitTypeVariantDeclarativeSignatureTemplate::new(
            db,
            parent_ty_template,
            parent_ty_template.self_ty(db),
        ))
    }
}
