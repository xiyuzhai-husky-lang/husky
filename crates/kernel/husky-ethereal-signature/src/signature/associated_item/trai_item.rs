mod associated_fn;
mod associated_val;
mod method_fn;
mod method_function;

pub use self::associated_fn::*;
pub use self::associated_val::*;
pub use self::method_fn::*;
pub use self::method_function::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum TraitItemEtherealSignatureTemplate {
    AssociatedFn(TraitAssociatedFnEtherealSignatureTemplate),
}

impl TraitItemEtherealSignatureTemplate {
    pub fn self_value_ty(self, db: &dyn EtherealSignatureDb) -> Option<EtherealTerm> {
        match self {
            TraitItemEtherealSignatureTemplate::AssociatedFn(_) => None,
        }
    }
}

impl HasEtherealSignatureTemplate for TraitItemPath {
    type EtherealSignatureTemplate = TraitItemEtherealSignatureTemplate;

    fn ethereal_signature_template(
        self,
        db: &dyn EtherealSignatureDb,
    ) -> EtherealSignatureResult<Self::EtherealSignatureTemplate> {
        todo!()
    }
}
