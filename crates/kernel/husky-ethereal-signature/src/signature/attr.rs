mod derive;

pub use self::derive::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = EtherealSignatureDb)]
#[enum_class::from_variants]
pub enum AttrEtherealSignatureTemplate {
    Derive(DeriveAttrEtherealSignatureTemplate),
}

impl HasEtherealSignatureTemplate for AttrPath {
    type EtherealSignatureTemplate = AttrEtherealSignatureTemplate;

    fn ethereal_signature_template(
        self,
        db: &dyn EtherealSignatureDb,
    ) -> EtherealSignatureResult<Self::EtherealSignatureTemplate> {
        attr_ethereal_signature_template(db, self)
    }
}

#[salsa::tracked(jar = EtherealSignatureJar)]
fn attr_ethereal_signature_template(
    db: &dyn EtherealSignatureDb,
    path: AttrPath,
) -> EtherealSignatureResult<AttrEtherealSignatureTemplate> {
    match path.declarative_signature_template(db)? {
        AttrDeclarativeSignatureTemplate::Derive(declarative_signature_template) => {
            DeriveAttrEtherealSignatureTemplate::from_declarative(
                db,
                declarative_signature_template,
            )
            .map(Into::into)
        }
    }
}
