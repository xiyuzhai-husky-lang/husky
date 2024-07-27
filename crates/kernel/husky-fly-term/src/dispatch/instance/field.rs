mod eth;
mod hol;
mod sol;

pub(crate) use self::eth::*;

pub type FlyFieldInstanceDispatch = FlyInstanceDispatch<FieldFlySignature>;

use super::*;
use crate::{dispatch::instance::method::HasFlyMethodDispatch, quary::FlyQuary};
use husky_coword::Ident;
use husky_entity_path::path::assoc_item::AssocItemPath;
use husky_entity_path::path::major_item::{trai::TraitPath, ty::TypePath};
use husky_eth_signature::signature::package::PackageEthSignatureData;
use husky_eth_signature::signature::{
    assoc_item::ty_item::memo::TypeMemoizedFieldEthSignature,
    major_item::ty::PropsFieldEthSignature,
};

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FieldFlySignature {
    PropsStruct {
        self_ty: FlyTerm,
        ty: FlyTerm,
    },
    Memoized {
        self_ty: FlyTerm,
        return_ty: FlyTerm,
        expr_ty: FlyTerm,
        path: AssocItemPath,
        instantiation: FlyInstantiation,
    },
}

impl FieldFlySignature {
    pub fn expr_ty(&self) -> FlyTerm {
        match *self {
            FieldFlySignature::PropsStruct { ty, .. } => ty,
            FieldFlySignature::Memoized { expr_ty, .. } => expr_ty,
        }
    }
}

impl IsInstanceItemFlySignature for FieldFlySignature {
    fn expr_ty(&self, self_value_final_quary: FlyQuary) -> FlyTermResult<FlyTerm> {
        // ad hoc
        // todo: consider field mutability
        Ok(match *self {
            FieldFlySignature::PropsStruct { ty, .. } => ty.with_quary(self_value_final_quary),
            FieldFlySignature::Memoized { expr_ty, .. } => expr_ty.with_quary(FlyQuary::Transient),
        })
    }

    type Path = TypePath;

    fn path(&self) -> Option<Self::Path> {
        // ad hoc
        None
    }

    fn instantiation(&self) -> Option<&FlyInstantiation> {
        // ad hoc
        None
    }
}

impl From<PropsFieldEthSignature> for FieldFlySignature {
    fn from(signature: PropsFieldEthSignature) -> Self {
        match signature {
            PropsFieldEthSignature::PropsStruct(signature) => FieldFlySignature::PropsStruct {
                self_ty: signature.self_ty().into(),
                ty: signature.ty().into(),
            },
        }
    }
}

impl From<TypeMemoizedFieldEthSignature> for FieldFlySignature {
    fn from(signature: TypeMemoizedFieldEthSignature) -> Self {
        FieldFlySignature::Memoized {
            self_ty: signature.return_ty().into(),
            return_ty: signature.return_ty().into(),
            expr_ty: signature.expr_ty().into(),
            path: signature.path().into(),
            instantiation: FlyInstantiation::from_eth(
                FlyInstantiationEnvironment::MemoizedField,
                signature.instantiation(),
            ),
        }
    }
}

impl FlyTerm {
    /// returns None if no such field
    pub fn field_dispatch(
        self,
        engine: &mut impl FlyTermEngineMut,
        ident: Ident,
        available_traits: &[TraitPath],
    ) -> FlyTermMaybeResult<FlyFieldInstanceDispatch> {
        self.field_dispatch_aux(
            engine,
            ident,
            available_traits,
            FlyIndirections::new(self.initial_place()),
        )
    }

    fn field_dispatch_aux<'db>(
        self,
        engine: &mut impl FlyTermEngineMut,
        ident: Ident,
        available_traits: &[TraitPath],
        indirections: FlyIndirections,
    ) -> FlyTermMaybeResult<FlyFieldInstanceDispatch> {
        match self.base_resolved(engine) {
            FlyTermBase::Eth(term) => {
                eth_ty_field_dispatch(engine.db(), term, ident, indirections, engine.context_ref())
            }
            FlyTermBase::Sol(term) => {
                term.field_dispatch_aux(engine, ident, available_traits, indirections)
            }
            FlyTermBase::Hol(term) => todo!(),
            FlyTermBase::Place => todo!(),
        }
    }
}
