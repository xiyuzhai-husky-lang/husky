pub(crate) mod method_fn;

use self::method_fn::MethodFnFlySignature;
use super::*;

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone)]
#[enum_class::from_variants]
pub enum MethodFlySignature {
    MethodFn(MethodFnFlySignature),
    MethodGn,
}

impl MemberSignature for MethodFlySignature {
    fn expr_ty(&self, self_value_final_place: FlyQuary) -> FlyTermResult<FlyTerm> {
        todo!()
    }
}

// impl From<&TraitForTypeMethodFnEtherealSignature> for MethodFlySignature {
//     fn from(sig: &TraitForTypeMethodFnEtherealSignature) -> Self {
//         MethodFlySignature::MethodFn(sig.into())
//     }
// }
