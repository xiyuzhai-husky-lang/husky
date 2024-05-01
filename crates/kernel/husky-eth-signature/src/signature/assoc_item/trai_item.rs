pub mod assoc_ritchie;
pub mod assoc_val;
pub mod method_curry;
pub mod method_ritchie;

use self::assoc_ritchie::*;
use self::method_ritchie::*;
use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum TraitItemEthTemplate {
    AssocRitchie(TraitAssocRitchieEthTemplate),
}

impl TraitItemEthTemplate {
    pub fn self_ty(self, _db: &::salsa::Db) -> Option<EthTerm> {
        match self {
            TraitItemEthTemplate::AssocRitchie(_) => None,
        }
    }
}

impl HasEthTemplate for TraitItemPath {
    type EthTemplate = TraitItemEthTemplate;

    fn eth_template(self, _db: &::salsa::Db) -> EtherealSignatureResult<Self::EthTemplate> {
        todo!()
    }
}
