pub mod backprop;
pub mod dep;
pub mod derive;
pub mod proj;
pub mod task;

use self::backprop::BackpropAttrEthTemplate;
use self::dep::DepAttrEthTemplate;
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
    Affect,
    Backprop(BackpropAttrEthTemplate),
    Dep(DepAttrEthTemplate),
    Derive(DeriveAttrEthTemplate),
    Task(TaskAttrEthTemplate),
    Test,
}

impl AttrEthTemplate {
    pub fn path(self, db: &::salsa::Db) -> AttrItemPath {
        match self {
            AttrEthTemplate::Affect => todo!(),
            AttrEthTemplate::Backprop(slf) => slf.path(db).into(),
            AttrEthTemplate::Dep(slf) => slf.path(db).into(),
            AttrEthTemplate::Derive(slf) => slf.path(db).into(),
            AttrEthTemplate::Task(slf) => slf.path(db).into(),
            AttrEthTemplate::Test => todo!(),
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
        AttrDecTemplate::Backprop(dec_template) => {
            BackpropAttrEthTemplate::from_dec(path, dec_template, db).map(Into::into)
        }
        AttrDecTemplate::Dep(dec_template) => {
            DepAttrEthTemplate::from_dec(path, dec_template, db).map(Into::into)
        }
        AttrDecTemplate::Derive(dec_template) => {
            DeriveAttrEthTemplate::from_dec(db, path, dec_template).map(Into::into)
        }
        AttrDecTemplate::Task(dec_template) => {
            TaskAttrEthTemplate::from_dec(db, path, dec_template).map(Into::into)
        }
        AttrDecTemplate::Proj(_) => todo!(),
        AttrDecTemplate::Singleton(_) => todo!(),
        AttrDecTemplate::Test => Ok(AttrEthTemplate::Test),
    }
}
