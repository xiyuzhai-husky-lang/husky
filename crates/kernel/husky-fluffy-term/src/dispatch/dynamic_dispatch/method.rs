mod ethereal;
mod hollow;
mod solid;

pub(crate) use self::ethereal::*;
pub(crate) use self::hollow::*;
pub(crate) use self::solid::*;

use super::*;
use husky_coword::Ident;
use husky_entity_syn_tree::helpers::{TraitInUseItemRecord, TraitInUseItemsWithGivenIdent};
use husky_regional_token::IdentRegionalToken;

impl MemberSignature for MethodFluffySignature {
    fn expr_ty(&self) -> FluffyTermResult<FluffyTerm> {
        todo!()
    }
}

pub type FluffyMethodDynamicDispatch = FluffyDynamicDispatch<MethodFluffySignature>;

pub trait HasFluffyTraitMethodDispatch: Copy {
    fn trai_method_dispatch(
        self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: SynExprIdx,
        ident_regional_token: IdentRegionalToken,
        mut indirections: FluffyTermDynamicDispatchIndirections,
    ) -> FluffyTermMaybeResult<FluffyMethodDynamicDispatch> {
        self.trai_method_dispatch_aux(
            engine,
            expr_idx,
            ident_regional_token,
            engine
                .trai_in_use_items_table()?
                .available_trait_items_with_given_ident(ident_regional_token.ident())?,
            indirections,
        )
    }

    fn trai_method_dispatch_aux(
        self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: SynExprIdx,
        ident_token: IdentRegionalToken,
        trai_item_records: TraitInUseItemsWithGivenIdent,
        indirections: FluffyTermDynamicDispatchIndirections,
    ) -> FluffyTermMaybeResult<FluffyMethodDynamicDispatch>;
}

pub trait HasFluffyTypeMethodDispatch: Copy {
    fn ty_method_dispatch(
        self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: SynExprIdx,
        ident_token: IdentRegionalToken,
        indirections: FluffyTermDynamicDispatchIndirections,
    ) -> FluffyTermMaybeResult<FluffyMethodDynamicDispatch>;
}

/// dispatch orders are
/// - builtin indirected type methods
/// - type methods
/// - custom indirected type methods
/// - builtin indirected trait methods
/// - trait methods
/// - custom indirected trait methods
pub trait HasFluffyMethodDispatch:
    HasFluffyTypeMethodDispatch + HasFluffyTraitMethodDispatch
{
    fn method_dispatch(
        self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: SynExprIdx,
        ident_token: IdentRegionalToken,
    ) -> FluffyTermMaybeResult<FluffyMethodDynamicDispatch> {
        let initial_place = self.initial_place();
        if let Some(dispatch) = self
            .ty_method_dispatch(
                engine,
                expr_idx,
                ident_token,
                FluffyTermDynamicDispatchIndirections::new(initial_place),
            )
            .into_result_option()?
        {
            return JustOk(dispatch);
        }
        self.trai_method_dispatch(
            engine,
            expr_idx,
            ident_token,
            FluffyTermDynamicDispatchIndirections::new(initial_place),
        )
    }

    fn initial_place(self) -> Place;
}

impl HasFluffyTypeMethodDispatch for FluffyTerm {
    fn ty_method_dispatch(
        self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: SynExprIdx,
        ident_token: IdentRegionalToken,
        indirections: FluffyTermDynamicDispatchIndirections,
    ) -> FluffyTermMaybeResult<FluffyMethodDynamicDispatch> {
        match self.base_resolved(engine) {
            FluffyTermBase::Ethereal(ty_term) => {
                ty_term.ty_method_dispatch(engine, expr_idx, ident_token, indirections)
            }
            FluffyTermBase::Solid(ty_term) => {
                ty_term.ty_method_dispatch(engine, expr_idx, ident_token, indirections)
            }
            FluffyTermBase::Hollow(ty_term) => {
                ty_term.ty_method_dispatch(engine, expr_idx, ident_token, indirections)
            }
            FluffyTermBase::Place => todo!(),
        }
    }
}

impl HasFluffyTraitMethodDispatch for FluffyTerm {
    fn trai_method_dispatch_aux(
        self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: SynExprIdx,
        ident_token: IdentRegionalToken,
        trai_item_records: TraitInUseItemsWithGivenIdent,
        indirections: FluffyTermDynamicDispatchIndirections,
    ) -> FluffyTermMaybeResult<FluffyMethodDynamicDispatch> {
        match self.base_resolved(engine) {
            FluffyTermBase::Ethereal(ty_term) => ty_term.trai_method_dispatch_aux(
                engine,
                expr_idx,
                ident_token,
                trai_item_records,
                indirections,
            ),
            FluffyTermBase::Solid(ty_term) => ty_term.trai_method_dispatch_aux(
                engine,
                expr_idx,
                ident_token,
                trai_item_records,
                indirections,
            ),
            FluffyTermBase::Hollow(ty_term) => ty_term.trai_method_dispatch_aux(
                engine,
                expr_idx,
                ident_token,
                trai_item_records,
                indirections,
            ),
            FluffyTermBase::Place => todo!(),
        }
    }
}

impl HasFluffyMethodDispatch for FluffyTerm {
    fn initial_place(self) -> Place {
        // ad hoc
        self.place().unwrap_or(Place::Transient)
    }
}

// fn method_dispatch_aux(
//     self,
//     engine: &mut impl FluffyTermEngine,
//     expr_idx: ExprIdx,
//     ident: Ident,
//     available_traits: &[TraitPath],
//     mut indirections: FluffyTermDynamicDispatchIndirections,
// ) -> FluffyTermMaybeResult<FluffyMethodDispatch> {
//     match self.base() {
//         FluffyTermBase::Ethereal(term) => {
//             todo!()
//             // ethereal_ty_method_dispatch(engine, expr_idx, term, ident)
//         }
//         FluffyTermBase::Solid(term) => {
//             term.method_dispatch_aux(engine, expr_idx, ident, available_traits, indirections)
//         }
//         FluffyTermBase::Hollow(term) => todo!(),
//     }
// }
