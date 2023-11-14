pub(crate) mod method_fn;

use self::method_fn::MethodFnFluffySignature;
use super::*;

#[derive(Debug, PartialEq, Eq, Clone)]
#[enum_class::from_variants]
pub enum MethodFluffySignature {
    MethodFn(MethodFnFluffySignature),
    MethodGn,
}

impl MemberSignature for MethodFluffySignature {
    fn expr_ty(&self) -> FluffyTermResult<FluffyTerm> {
        todo!()
    }
}

impl From<&TraitForTypeMethodFnEtherealSignature> for MethodFluffySignature {
    fn from(sig: &TraitForTypeMethodFnEtherealSignature) -> Self {
        MethodFluffySignature::MethodFn(sig.into())
    }
}
