mod ethereal;
mod hollow;
mod solid;

use super::*;
use assoc_item::{
    trai_for_ty_item::method_ritchie::TraitForTypeMethodRitchieFlySignature,
    ty_item::method_ritchie::TypeMethodRitchieFlySignature,
};
use husky_coword::Ident;
use husky_entity_tree::helpers::AvailableTraitItemsWithGivenIdent;
use husky_eth_signature::{error::EthSignatureResult, signature::package::PackageEthSignatureData};
use husky_regional_token::IdentRegionalToken;
use path::assoc_item::AssocItemPath;

pub type MethodFlyInstanceDispatch = FlyInstanceDispatch<MethodFlySignature>;

impl MethodFlyInstanceDispatch {
    pub fn requires_lazy_to_use(&self, db: &::salsa::Db) -> bool {
        match self.signature {
            MethodFlySignature::TypeMethodRitchie(ref signature) => {
                signature.path.entity_kind(db).requires_lazy_to_use()
            }
            MethodFlySignature::TraitForTypeMethodRitchie(ref signature) => {
                signature.path.entity_kind(db).requires_lazy_to_use()
            }
        }
    }
}

#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq)]
pub enum MethodFlySignature {
    TypeMethodRitchie(TypeMethodRitchieFlySignature),
    TraitForTypeMethodRitchie(TraitForTypeMethodRitchieFlySignature),
}

impl MethodFlySignature {
    pub fn path(&self) -> AssocItemPath {
        match self {
            MethodFlySignature::TypeMethodRitchie(slf) => slf.path().into(),
            MethodFlySignature::TraitForTypeMethodRitchie(slf) => slf.path().into(),
        }
    }

    pub fn instantiation(&self) -> &FlyInstantiation {
        match self {
            MethodFlySignature::TypeMethodRitchie(slf) => slf.instantiation(),
            MethodFlySignature::TraitForTypeMethodRitchie(slf) => slf.instantiation(),
        }
    }
}

impl IsInstanceItemFlySignature for MethodFlySignature {
    fn expr_ty(&self, self_value_final_place: FlyQuary) -> FlyTermResult<FlyTerm> {
        match self {
            MethodFlySignature::TypeMethodRitchie(_) => todo!(),
            MethodFlySignature::TraitForTypeMethodRitchie(_) => todo!(),
        }
    }

    type Path = AssocItemPath;

    fn path(&self) -> Option<Self::Path> {
        Some(match self {
            MethodFlySignature::TypeMethodRitchie(slf) => slf.path.into(),
            MethodFlySignature::TraitForTypeMethodRitchie(_) => todo!(),
        })
    }

    fn instantiation(&self) -> Option<&FlyInstantiation> {
        todo!()
    }
}

pub trait HasFlyTraitMethodDispatch: Copy {
    fn trai_method_dispatch(
        self,
        engine: &mut impl FlyTermEngineMut,
        expr_idx: SynExprIdx,
        ident_regional_token: IdentRegionalToken,
        indirections: FlyIndirections,
    ) -> FlyTermMaybeResult<MethodFlyInstanceDispatch> {
        self.trai_method_dispatch_aux(
            engine,
            expr_idx,
            ident_regional_token,
            engine
                .available_trai_items_table()
                .available_trait_items_with_given_ident(ident_regional_token.ident())?,
            indirections,
        )
    }

    fn trai_method_dispatch_aux(
        self,
        engine: &mut impl FlyTermEngineMut,
        expr_idx: SynExprIdx,
        ident_token: IdentRegionalToken,
        trai_item_records: AvailableTraitItemsWithGivenIdent,
        indirections: FlyIndirections,
    ) -> FlyTermMaybeResult<MethodFlyInstanceDispatch>;
}

pub trait HasFlyTypeMethodDispatch: Copy {
    fn ty_method_dispatch<'db>(
        self,
        engine: &mut impl FlyTermEngineMut,
        expr_idx: SynExprIdx,
        ident_token: IdentRegionalToken,
        indirections: FlyIndirections,
    ) -> FlyTermMaybeResult<MethodFlyInstanceDispatch>;
}

/// dispatch orders are
/// - builtin indirected type methods
/// - type methods
/// - custom indirected type methods
/// - builtin indirected trait methods
/// - trait methods
/// - custom indirected trait methods
pub trait HasFlyMethodDispatch: HasFlyTypeMethodDispatch + HasFlyTraitMethodDispatch {
    fn method_dispatch(
        self,
        engine: &mut impl FlyTermEngineMut,
        expr_idx: SynExprIdx,
        ident_token: IdentRegionalToken,
    ) -> FlyTermMaybeResult<MethodFlyInstanceDispatch> {
        let initial_place = self.initial_place();
        if let Some(dispatch) = self
            .ty_method_dispatch(
                engine,
                expr_idx,
                ident_token,
                FlyIndirections::new(initial_place),
            )
            .into_result_option()?
        {
            return JustOk(dispatch);
        }
        self.trai_method_dispatch(
            engine,
            expr_idx,
            ident_token,
            FlyIndirections::new(initial_place),
        )
    }

    fn initial_place(self) -> FlyQuary;
}

impl HasFlyTypeMethodDispatch for FlyTerm {
    fn ty_method_dispatch(
        self,
        engine: &mut impl FlyTermEngineMut,
        expr_idx: SynExprIdx,
        ident_token: IdentRegionalToken,
        indirections: FlyIndirections,
    ) -> FlyTermMaybeResult<MethodFlyInstanceDispatch> {
        match self.base_resolved(engine) {
            FlyTermBase::Eth(ty_term) => {
                ty_term.ty_method_dispatch(engine, expr_idx, ident_token, indirections)
            }
            FlyTermBase::Sol(ty_term) => {
                ty_term.ty_method_dispatch(engine, expr_idx, ident_token, indirections)
            }
            FlyTermBase::Hol(ty_term) => {
                ty_term.ty_method_dispatch(engine, expr_idx, ident_token, indirections)
            }
            FlyTermBase::Place => todo!(),
        }
    }
}

impl HasFlyTraitMethodDispatch for FlyTerm {
    fn trai_method_dispatch_aux(
        self,
        engine: &mut impl FlyTermEngineMut,
        expr_idx: SynExprIdx,
        ident_token: IdentRegionalToken,
        trai_item_records: AvailableTraitItemsWithGivenIdent,
        indirections: FlyIndirections,
    ) -> FlyTermMaybeResult<MethodFlyInstanceDispatch> {
        match self.base_resolved(engine) {
            FlyTermBase::Eth(ty_term) => ty_term.trai_method_dispatch_aux(
                engine,
                expr_idx,
                ident_token,
                trai_item_records,
                indirections,
            ),
            FlyTermBase::Sol(ty_term) => ty_term.trai_method_dispatch_aux(
                engine,
                expr_idx,
                ident_token,
                trai_item_records,
                indirections,
            ),
            FlyTermBase::Hol(ty_term) => ty_term.trai_method_dispatch_aux(
                engine,
                expr_idx,
                ident_token,
                trai_item_records,
                indirections,
            ),
            FlyTermBase::Place => todo!(),
        }
    }
}

impl HasFlyMethodDispatch for FlyTerm {
    fn initial_place(self) -> FlyQuary {
        self.initial_place()
    }
}

impl FlyTerm {
    pub(crate) fn initial_place(self) -> FlyQuary {
        // ad hoc
        self.quary().unwrap_or(FlyQuary::Transient)
    }
}
