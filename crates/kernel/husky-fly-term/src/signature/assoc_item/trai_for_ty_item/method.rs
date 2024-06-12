pub(crate) mod method_ritchie;

use self::method_ritchie::TraitForTypeMethodRitchieFlySignature;
use super::*;
use crate::quary::FlyQuary;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone)]
#[enum_class::from_variants]
pub enum TraitForTypeMethodFlySignature {
    MethodRitchie(TraitForTypeMethodRitchieFlySignature),
    MethodGn,
}

impl IsInstanceItemFlySignature for TraitForTypeMethodFlySignature {
    fn expr_ty(&self, self_value_final_place: FlyQuary) -> FlyTermResult<FlyTerm> {
        todo!()
    }
}

// impl From<&TraitForTypeMethodRitchieEtherealSignature> for MethodFlySignature {
//     fn from(sig: &TraitForTypeMethodRitchieEtherealSignature) -> Self {
//         MethodFlySignature::MethodFn(sig.into())
//     }
// }
