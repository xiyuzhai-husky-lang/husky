pub mod derive;

use self::derive::*;
use super::*;
use husky_dec_signature::signature::attr::AttrDecTemplate;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
#[non_exhaustive]
pub enum AttrEthTemplate {
    Derive(DeriveAttrEthTemplate),
    Effect,
}

impl HasEthTemplate for AttrItemPath {
    type EthTemplate = AttrEthTemplate;

    fn eth_template(self, db: &::salsa::Db) -> EtherealSignatureResult<Self::EthTemplate> {
        attr_eth_template(db, self)
    }
}

// #[salsa::tracked]
fn attr_eth_template(
    db: &::salsa::Db,
    path: AttrItemPath,
) -> EtherealSignatureResult<AttrEthTemplate> {
    match path.dec_template(db)? {
        AttrDecTemplate::Derive(dec_template) => {
            DeriveAttrEthTemplate::from_dec(db, dec_template).map(Into::into)
        }
    }
}
