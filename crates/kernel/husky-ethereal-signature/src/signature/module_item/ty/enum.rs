use super::*;
use husky_declarative_signature::EnumDeclarativeSignatureTemplate;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct EnumEtherealSignatureTemplate {
    #[return_ref]
    pub generic_parameters: EtherealGenericParameters,
}

impl EnumEtherealSignatureTemplate {
    pub(super) fn from_declarative(
        db: &dyn EtherealSignatureDb,
        declarative_signature_template: EnumDeclarativeSignatureTemplate,
    ) -> EtherealSignatureResult<Self> {
        let generic_parameters = EtherealGenericParameters::from_declarative(
            db,
            declarative_signature_template.generic_parameters(db),
        )?;
        Ok(Self::new(db, generic_parameters))
    }
}
