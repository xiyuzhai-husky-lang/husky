use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct EnumUnitTypeVariantEtherealSignatureTemplate {
    pub parent_ty_template: EnumTypeEtherealSignatureTemplate,
}

impl EnumUnitTypeVariantEtherealSignatureTemplate {
    pub(super) fn from_declarative(
        db: &dyn EtherealSignatureDb,
        path: TypeVariantPath,
        _declarative_signature_template: EnumUnitTypeVariantDeclarativeSignatureTemplate,
    ) -> EtherealSignatureResult<Self> {
        let TypeEtherealSignatureTemplate::Enum(parent_ty_template) =
            path.parent_ty_path(db).ethereal_signature_template(db)?
        else {
            unreachable!()
        };
        Ok(Self::new(db, parent_ty_template))
    }
}
