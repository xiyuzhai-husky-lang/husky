use super::*;
use husky_entity_path::path::{assoc_item::AssocItemPath, major_item::ty::TypePath};
use husky_eth_signature::{
    context::EthSignatureBuilderContextItd,
    signature::{
        assoc_item::trai_for_ty_item::method_ritchie::{
            TraitForTypeMethodRitchieEthSignature, TraitForTypeMethodRitchieEthTemplate,
        },
        HasEthTemplate,
    },
};
use husky_eth_term::term::symbolic_variable::EthTermSymbolIndexImpl;
use husky_regional_token::IdentRegionalToken;
use quary::FlyQuary;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TraitForTypeMethodRitchieFlySignature {
    pub path: TraitForTypeItemPath,
    pub self_value_parameter: FlyRitchieSimpleParameter,
    pub parenate_parameters: SmallVec<[FlyRitchieParameter; 4]>,
    pub return_ty: FlyTerm,
    pub instantiation: FlyInstantiation,
}

impl IsInstanceItemFlySignature for TraitForTypeMethodRitchieFlySignature {
    fn expr_ty(&self, self_value_final_place: FlyQuary) -> FlyTermResult<FlyTerm> {
        todo!()
    }

    type Path = TraitForTypeItemPath;

    fn path(&self) -> Option<Self::Path> {
        Some(self.path)
    }

    fn instantiation(&self) -> Option<&FlyInstantiation> {
        Some(&self.instantiation)
    }
}

impl TraitForTypeMethodRitchieFlySignature {
    pub(crate) fn from_eth(
        self_place: FlyQuary,
        eth_sig: &TraitForTypeMethodRitchieEthSignature,
    ) -> Self {
        Self {
            path: eth_sig.path().into(),
            self_value_parameter: eth_sig.self_value_parameter.into(),
            parenate_parameters: eth_sig
                .parenate_parameters()
                .iter()
                .map(|&param| param.into())
                .collect(),
            return_ty: eth_sig.return_ty().into(),
            instantiation: FlyInstantiation::from_eth(
                FlyInstantiationEnvironment::MethodFn { self_place },
                eth_sig.instantiation(),
            ),
        }
    }

    pub fn instantiation(&self) -> &FlyInstantiation {
        &self.instantiation
    }
}

impl TraitForTypeMethodRitchieFlySignature {
    pub fn nonself_parameter_contracted_tys(&self) -> &[FlyRitchieParameter] {
        &self.parenate_parameters
    }

    pub fn return_ty(&self) -> FlyTerm {
        self.return_ty
    }

    pub fn path(&self) -> TraitForTypeItemPath {
        self.path
    }
}
