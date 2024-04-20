mod assoc_ritchie;
mod assoc_val;
mod method_curry;
mod method_ritchie;

pub use self::assoc_ritchie::*;

pub use self::method_ritchie::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum TraitItemEthTemplate {
    AssocFn(TraitAssocFnEthTemplate),
}

impl TraitItemEthTemplate {
    pub fn self_ty(self, _db: &::salsa::Db) -> Option<EthTerm> {
        match self {
            TraitItemEthTemplate::AssocFn(_) => None,
        }
    }
}

impl HasEthTemplate for TraitItemPath {
    type EthTemplate = TraitItemEthTemplate;

    fn eth_template(self, _db: &::salsa::Db) -> EtherealSignatureResult<Self::EthTemplate> {
        todo!()
    }
}
