use super::*;
use husky_declarative_signature::EnumDeclarativeSignatureTemplate;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct EnumEtherealSignatureTemplate {}

impl EnumEtherealSignatureTemplate {
    pub(super) fn from_declarative(
        db: &dyn EtherealSignatureDb,
        declarative_signature_template: EnumDeclarativeSignatureTemplate,
    ) -> EtherealSignatureResult<Self> {
        Ok(Self::new(db))
    }
}
