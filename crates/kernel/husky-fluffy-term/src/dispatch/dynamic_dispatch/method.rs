mod ethereal;
mod hollow;
mod solid;

pub(crate) use self::ethereal::*;
pub(crate) use self::hollow::*;
pub(crate) use self::solid::*;

use super::*;
use husky_coword::Ident;
use husky_entity_tree::{TraitInUseItemRecord, TraitInUseItemsWithGivenIdent};
use husky_token::IdentToken;

impl MemberSignature for MethodFluffySignature {
    fn expr_ty(
        &self,
        indirections: &[FluffyDynamicDispatchIndirection],
    ) -> FluffyTermResult<FluffyTerm> {
        todo!()
    }
}

pub type FluffyMethodDispatch = FluffyDynamicDispatch<MethodFluffySignature>;

pub trait HasFluffyTraitMethodDispatch: Copy {
    fn trai_method_dispatch(
        self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: SynExprIdx,
        ident_token: IdentToken,
    ) -> FluffyTermMaybeResult<FluffyMethodDispatch> {
        self.trai_method_dispatch_aux(
            engine,
            expr_idx,
            ident_token,
            engine
                .trai_in_use_items_table()?
                .available_trait_items_with_given_ident(ident_token.ident())?,
        )
    }

    fn trai_method_dispatch_aux(
        self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: SynExprIdx,
        ident_token: IdentToken,
        trai_item_records: TraitInUseItemsWithGivenIdent,
    ) -> FluffyTermMaybeResult<FluffyMethodDispatch>;
}

pub trait HasFluffyTypeMethodDispatch: Copy {
    fn ty_method_dispatch(
        self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: SynExprIdx,
        ident_token: IdentToken,
    ) -> FluffyTermMaybeResult<FluffyMethodDispatch>;
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
        ident_token: IdentToken,
    ) -> FluffyTermMaybeResult<FluffyMethodDispatch> {
        if let Some(dispatch) = self
            .ty_method_dispatch(engine, expr_idx, ident_token)
            .into_result_option()?
        {
            return JustOk(dispatch);
        }
        self.trai_method_dispatch(engine, expr_idx, ident_token)
    }
}

impl HasFluffyTypeMethodDispatch for FluffyTerm {
    fn ty_method_dispatch(
        self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: SynExprIdx,
        ident_token: IdentToken,
    ) -> FluffyTermMaybeResult<FluffyMethodDispatch> {
        match self.nested() {
            NestedFluffyTerm::Ethereal(ty_term) => {
                ty_term.ty_method_dispatch(engine, expr_idx, ident_token)
            }
            NestedFluffyTerm::Solid(ty_term) => {
                ty_term.ty_method_dispatch(engine, expr_idx, ident_token)
            }
            NestedFluffyTerm::Hollow(ty_term) => {
                ty_term.ty_method_dispatch(engine, expr_idx, ident_token)
            }
        }
    }
}

impl HasFluffyTraitMethodDispatch for FluffyTerm {
    fn trai_method_dispatch_aux(
        self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: SynExprIdx,
        ident_token: IdentToken,
        trai_item_records: TraitInUseItemsWithGivenIdent,
    ) -> FluffyTermMaybeResult<FluffyMethodDispatch> {
        match self.nested() {
            NestedFluffyTerm::Ethereal(ty_term) => {
                ty_term.trai_method_dispatch_aux(engine, expr_idx, ident_token, trai_item_records)
            }
            NestedFluffyTerm::Solid(ty_term) => {
                ty_term.trai_method_dispatch_aux(engine, expr_idx, ident_token, trai_item_records)
            }
            NestedFluffyTerm::Hollow(ty_term) => {
                ty_term.trai_method_dispatch_aux(engine, expr_idx, ident_token, trai_item_records)
            }
        }
    }
}

impl HasFluffyMethodDispatch for FluffyTerm {}

// fn method_dispatch_aux(
//     self,
//     engine: &mut impl FluffyTermEngine,
//     expr_idx: ExprIdx,
//     ident: Ident,
//     available_traits: &[TraitPath],
//     mut indirections: SmallVec<[FluffyDynamicDispatchIndirection; 2]>,
// ) -> FluffyTermMaybeResult<FluffyMethodDispatch> {
//     match self.nested() {
//         NestedFluffyTerm::Ethereal(term) => {
//             todo!()
//             // ethereal_ty_method_dispatch(engine, expr_idx, term, ident)
//         }
//         NestedFluffyTerm::Solid(term) => {
//             term.method_dispatch_aux(engine, expr_idx, ident, available_traits, indirections)
//         }
//         NestedFluffyTerm::Hollow(term) => todo!(),
//     }
// }
