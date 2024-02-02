mod ethereal;
mod hollow;
mod solid;

use super::*;
use husky_coword::Ident;
use husky_entity_tree::helpers::TraitInUseItemsWithGivenIdent;
use husky_regional_token::IdentRegionalToken;

pub type FlyMethodDynamicDispatch = FlyDynamicDispatch<MethodFlySignature>;

pub trait HasFlyTraitMethodDispatch: Copy {
    fn trai_method_dispatch(
        self,
        engine: &mut impl FlyTermEngine,
        expr_idx: SynExprIdx,
        ident_regional_token: IdentRegionalToken,
        indirections: FlyIndirections,
    ) -> FlyTermMaybeResult<FlyMethodDynamicDispatch> {
        self.trai_method_dispatch_aux(
            engine,
            expr_idx,
            ident_regional_token,
            engine
                .trai_in_use_items_table()
                .available_trait_items_with_given_ident(ident_regional_token.ident())?,
            indirections,
        )
    }

    fn trai_method_dispatch_aux(
        self,
        engine: &mut impl FlyTermEngine,
        expr_idx: SynExprIdx,
        ident_token: IdentRegionalToken,
        trai_item_records: TraitInUseItemsWithGivenIdent,
        indirections: FlyIndirections,
    ) -> FlyTermMaybeResult<FlyMethodDynamicDispatch>;
}

pub trait HasFlyTypeMethodDispatch: Copy {
    fn ty_method_dispatch(
        self,
        engine: &mut impl FlyTermEngine,
        expr_idx: SynExprIdx,
        ident_token: IdentRegionalToken,
        indirections: FlyIndirections,
    ) -> FlyTermMaybeResult<FlyMethodDynamicDispatch>;
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
        engine: &mut impl FlyTermEngine,
        expr_idx: SynExprIdx,
        ident_token: IdentRegionalToken,
    ) -> FlyTermMaybeResult<FlyMethodDynamicDispatch> {
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

    fn initial_place(self) -> FlyPlace;
}

impl HasFlyTypeMethodDispatch for FlyTerm {
    fn ty_method_dispatch(
        self,
        engine: &mut impl FlyTermEngine,
        expr_idx: SynExprIdx,
        ident_token: IdentRegionalToken,
        indirections: FlyIndirections,
    ) -> FlyTermMaybeResult<FlyMethodDynamicDispatch> {
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
        engine: &mut impl FlyTermEngine,
        expr_idx: SynExprIdx,
        ident_token: IdentRegionalToken,
        trai_item_records: TraitInUseItemsWithGivenIdent,
        indirections: FlyIndirections,
    ) -> FlyTermMaybeResult<FlyMethodDynamicDispatch> {
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
    fn initial_place(self) -> FlyPlace {
        // ad hoc
        self.place().unwrap_or(FlyPlace::Transient)
    }
}
