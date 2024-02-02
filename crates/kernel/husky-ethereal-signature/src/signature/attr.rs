mod derive;

pub use self::derive::*;

use super::*;

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
#[non_exhaustive]
pub enum AttrEthTemplate {
    Derive(DeriveAttrEthTemplate),
}

impl HasEthTemplate for AttrItemPath {
    type EthTemplate = AttrEthTemplate;

    fn ethereal_signature_template(
        self,
        db: &::salsa::Db,
    ) -> EtherealSignatureResult<Self::EthTemplate> {
        attr_ethereal_signature_template(db, self)
    }
}

// #[salsa::tracked(jar = EtherealSignatureJar)]
fn attr_ethereal_signature_template(
    db: &::salsa::Db,
    path: AttrItemPath,
) -> EtherealSignatureResult<AttrEthTemplate> {
    match path.dec_template(db)? {
        AttrDecTemplate::Derive(dec_template) => {
            DeriveAttrEthTemplate::from_declarative(db, dec_template).map(Into::into)
        }
    }
}
