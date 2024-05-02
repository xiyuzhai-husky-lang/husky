pub mod form;
pub mod trai;
pub mod ty;

use self::form::*;
use self::trai::*;
use self::ty::*;
use super::*;
use husky_entity_path::path::major_item::MajorItemPath;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum MajorItemEthTemplate {
    Type(TypeEthTemplate),
    Form(FormEthTemplate),
    Trait(TraitEthTemplate),
}

impl HasEthTemplate for MajorItemPath {
    type EthTemplate = MajorItemEthTemplate;

    fn eth_template(self, db: &::salsa::Db) -> EtherealSignatureResult<Self::EthTemplate> {
        Ok(match self {
            MajorItemPath::Type(path) => path.eth_template(db)?.into(),
            MajorItemPath::Trait(path) => path.eth_template(db)?.into(),
            MajorItemPath::Form(path) => path.eth_template(db)?.into(),
        })
    }
}
