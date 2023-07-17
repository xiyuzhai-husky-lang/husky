mod ethereal;
mod hollow;
mod solid;

pub(crate) use self::ethereal::*;
pub(crate) use self::hollow::*;
pub(crate) use self::solid::*;

use super::*;
use husky_coword::Ident;

impl MemberSignature for MethodFluffySignature {
    fn expr_ty(
        &self,
        indirections: &[FluffyDynamicDispatchIndirection],
    ) -> FluffyTermResult<FluffyTerm> {
        todo!()
    }
}

pub type FluffyMethodDispatch = FluffyDynamicDispatch<MethodFluffySignature>;

impl FluffyTerm {
    pub fn method_dispatch(
        self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: ExprIdx,
        ident: Ident,
        available_traits: &[TraitPath],
    ) -> FluffyTermMaybeResult<FluffyMethodDispatch> {
        self.method_dispatch_aux(engine, expr_idx, ident, available_traits, smallvec![])
    }

    fn method_dispatch_aux(
        self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: ExprIdx,
        ident: Ident,
        available_traits: &[TraitPath],
        mut indirections: SmallVec<[FluffyDynamicDispatchIndirection; 2]>,
    ) -> FluffyTermMaybeResult<FluffyMethodDispatch> {
        match self.nested() {
            NestedFluffyTerm::Ethereal(term) => {
                ethereal_ty_method_dispatch(engine, expr_idx, term, ident)
            }
            NestedFluffyTerm::Solid(term) => {
                term.method_dispatch_aux(engine, expr_idx, ident, available_traits, indirections)
            }
            NestedFluffyTerm::Hollow(term) => todo!(),
        }
    }
}
