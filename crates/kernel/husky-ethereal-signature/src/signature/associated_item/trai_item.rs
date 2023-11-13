mod associated_fn;
mod associated_val;
mod method_fn;
mod method_function;

pub use self::associated_fn::*;

pub use self::method_fn::*;


use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum TraitItemEtherealSignatureTemplate {
    AssociatedFn(TraitAssociatedFnEtherealSignatureTemplate),
}

impl TraitItemEtherealSignatureTemplate {
    pub fn self_ty(self, _db: &dyn EtherealSignatureDb) -> Option<EtherealTerm> {
        match self {
            TraitItemEtherealSignatureTemplate::AssociatedFn(_) => None,
        }
    }
}

impl HasEtherealSignatureTemplate for TraitItemPath {
    type EtherealSignatureTemplate = TraitItemEtherealSignatureTemplate;

    fn ethereal_signature_template(
        self,
        _db: &dyn EtherealSignatureDb,
    ) -> EtherealSignatureResult<Self::EtherealSignatureTemplate> {
        todo!()
    }
}
