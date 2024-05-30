pub mod assoc_ritchie;
pub mod assoc_val;
pub mod method_curry;
pub mod method_ritchie;

use self::assoc_ritchie::*;
use super::*;
use husky_entity_path::path::assoc_item::trai_item::TraitItemPath;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum TraitItemEthTemplate {
    AssocRitchie(TraitAssocRitchieEthTemplate),
}

impl TraitItemEthTemplate {
    pub fn path(self, db: &::salsa::Db) -> TraitItemPath {
        match self {
            TraitItemEthTemplate::AssocRitchie(slf) => slf.path(db),
        }
    }

    pub fn self_ty(self, _db: &::salsa::Db) -> Option<EthTerm> {
        match self {
            TraitItemEthTemplate::AssocRitchie(_) => None,
        }
    }
}

impl HasEthTemplate for TraitItemPath {
    type EthTemplate = TraitItemEthTemplate;

    fn eth_template(self, _db: &::salsa::Db) -> EthSignatureResult<Self::EthTemplate> {
        todo!()
    }
}
