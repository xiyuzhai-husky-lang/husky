mod derive;

pub use self::derive::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = EtherealSignatureDb)]
#[enum_class::from_variants]
pub enum DecrEtherealSignatureTemplate {
    Derive(DeriveDecrEtherealSignatureTemplate),
}

impl HasEtherealSignatureTemplate for DecrPath {
    type EtherealSignatureTemplate = DecrEtherealSignatureTemplate;

    fn ethereal_signature_template(
        self,
        db: &dyn EtherealSignatureDb,
    ) -> EtherealSignatureResult<Self::EtherealSignatureTemplate> {
        decr_ethereal_signature_template(db, self)
    }
}

#[salsa::tracked(jar = EtherealSignatureJar)]
fn decr_ethereal_signature_template(
    db: &dyn EtherealSignatureDb,
    path: DecrPath,
) -> EtherealSignatureResult<DecrEtherealSignatureTemplate> {
    match path.declarative_signature_template(db)? {
        DecrDeclarativeSignatureTemplate::Derive(declarative_signature_template) => {
            DeriveDecrEtherealSignatureTemplate::from_declarative(
                db,
                declarative_signature_template,
            )
            .map(Into::into)
        }
    }
}
