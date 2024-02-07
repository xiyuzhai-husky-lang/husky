mod assoc_fn;
mod assoc_val;
mod method_fn;
mod method_function;

pub use self::assoc_fn::*;

pub use self::method_fn::*;

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
