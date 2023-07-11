use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct EnumTupleTypeVariantEtherealSignatureTemplate {
    pub parent_ty_template: EnumEtherealSignatureTemplate,
}

impl EnumTupleTypeVariantEtherealSignatureTemplate {
    pub(super) fn from_declarative(
        db: &dyn EtherealSignatureDb,
        path: TypeVariantPath,
        declarative_signature_template: EnumTupleTypeVariantDeclarativeSignatureTemplate,
    ) -> EtherealSignatureResult<Self> {
        let TypeEtherealSignatureTemplate::Enum(parent_ty_template) =
                path.parent_ty_path(db).ethereal_signature_template(db)? else {
            unreachable!()
        };
        Ok(Self::new(db, parent_ty_template))
    }
}
