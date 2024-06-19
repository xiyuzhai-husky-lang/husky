pub mod derive;
pub mod task;

use self::derive::*;
use self::task::*;
use super::*;
use husky_dec_signature::signature::attr::AttrDecTemplate;
use husky_entity_path::path::attr::AttrItemPath;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
#[non_exhaustive]
pub enum AttrEthTemplate {
    Derive(DeriveAttrEthTemplate),
    Affect,
    Task(TaskAttrEthTemplate),
}

impl AttrEthTemplate {
    pub fn path(self, db: &::salsa::Db) -> AttrItemPath {
        match self {
            AttrEthTemplate::Derive(slf) => slf.path(db).into(),
            AttrEthTemplate::Affect => todo!(),
            AttrEthTemplate::Task(slf) => slf.path(db).into(),
        }
    }
}

impl HasEthTemplate for AttrItemPath {
    type EthTemplate = AttrEthTemplate;

    fn eth_template(self, db: &::salsa::Db) -> EthSignatureResult<Self::EthTemplate> {
        attr_eth_template(db, self)
    }
}

#[salsa::tracked]
fn attr_eth_template(db: &::salsa::Db, path: AttrItemPath) -> EthSignatureResult<AttrEthTemplate> {
    match path.dec_template(db)? {
        AttrDecTemplate::Deps(_) => todo!(),
        AttrDecTemplate::Derive(dec_template) => {
            DeriveAttrEthTemplate::from_dec(db, path, dec_template).map(Into::into)
        }
        AttrDecTemplate::Task(dec_template) => {
            TaskAttrEthTemplate::from_dec(db, path, dec_template).map(Into::into)
        }
        AttrDecTemplate::Projection(_) => todo!(),
        AttrDecTemplate::Singleton(_) => todo!(),
    }
}
