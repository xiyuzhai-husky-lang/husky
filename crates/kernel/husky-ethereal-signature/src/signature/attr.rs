mod derive;

pub use self::derive::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
#[enum_class::from_variants]
pub enum AttrEtherealSignatureTemplate {
    Derive(DeriveAttrEtherealSignatureTemplate),
}

impl HasEtherealSignatureTemplate for AttrItemPath {
    type EtherealSignatureTemplate = AttrEtherealSignatureTemplate;

    fn ethereal_signature_template(
        self,
        db: &::salsa::Db,
    ) -> EtherealSignatureResult<Self::EtherealSignatureTemplate> {
        attr_ethereal_signature_template(db, self)
    }
}

// #[salsa::tracked(jar = EtherealSignatureJar)]
fn attr_ethereal_signature_template(
    db: &::salsa::Db,
    path: AttrItemPath,
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
