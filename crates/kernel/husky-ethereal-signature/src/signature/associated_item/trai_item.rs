mod associated_fn;
mod associated_val;
mod method_fn;
mod method_function;

pub use self::associated_fn::*;

pub use self::method_fn::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum TraitItemEthTemplate {
    AssociatedFn(TraitAssociatedFnEthTemplate),
}

impl TraitItemEthTemplate {
    pub fn self_ty(self, _db: &::salsa::Db) -> Option<EtherealTerm> {
        match self {
            TraitItemEthTemplate::AssociatedFn(_) => None,
        }
    }
}

impl HasEthTemplate for TraitItemPath {
    type EthTemplate = TraitItemEthTemplate;

    fn ethereal_signature_template(
        self,
        _db: &::salsa::Db,
    ) -> EtherealSignatureResult<Self::EthTemplate> {
        todo!()
    }
}
