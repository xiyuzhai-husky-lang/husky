use super::*;
use husky_declarative_signature::EnumDeclarativeSignatureTemplate;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct EnumEtherealSignatureTemplate {}

impl HasEtherealSignatureTemplate for EnumDeclarativeSignatureTemplate {
    type EtherealSignatureTemplate = EnumEtherealSignatureTemplate;

    fn ethereal_signature_template(
        self,
        db: &dyn EtherealSignatureDb,
    ) -> EtherealSignatureResult<Self::EtherealSignatureTemplate> {
        enum_ethereal_signature_template(db, self)
    }
}

#[salsa::tracked(jar = EtherealSignatureJar)]
pub(crate) fn enum_ethereal_signature_template(
    db: &dyn EtherealSignatureDb,
    declarative_signature: EnumDeclarativeSignatureTemplate,
) -> EtherealSignatureResult<EnumEtherealSignatureTemplate> {
    todo!()
}
