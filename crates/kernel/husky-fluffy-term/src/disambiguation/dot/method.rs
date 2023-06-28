mod ethereal;
mod hollow;
mod solid;

pub(crate) use self::ethereal::*;
pub(crate) use self::hollow::*;
pub(crate) use self::solid::*;

// mod dyn_trai;
// mod trai_for_ty;
// mod ty;

// pub use self::trai_for_ty::*;
// pub use self::ty::*;

use super::*;
use husky_ethereal_signature::EtherealMethodSignature;
use husky_word::Ident;

impl MemberSignature for MethodFluffySignature {
    fn expr_ty(&self, indirections: &[FluffyDotIndirection]) -> FluffyTermResult<FluffyTerm> {
        todo!()
    }
}

pub type FluffyMethodDisambiguation = FluffyDotDisambiguation<MethodFluffySignature>;

impl FluffyTerm {
    pub fn method_disambiguation(
        self,
        engine: &mut impl FluffyTermEngine,
        ident: Ident,
        available_traits: &[TraitPath],
    ) -> FluffyTermMaybeResult<FluffyMethodDisambiguation> {
        self.method_disambiguation_aux(engine, ident, available_traits, smallvec![])
    }

    fn method_disambiguation_aux(
        self,
        engine: &mut impl FluffyTermEngine,
        ident: Ident,
        available_traits: &[TraitPath],
        mut indirections: SmallVec<[FluffyDotIndirection; 2]>,
    ) -> FluffyTermMaybeResult<FluffyMethodDisambiguation> {
        match self.nested() {
            NestedFluffyTerm::Ethereal(term) => {
                ethereal_ty_method_disambiguation(engine, term, ident)
            }
            NestedFluffyTerm::Solid(term) => {
                term.method_disambiguation_aux(engine, ident, available_traits, indirections)
            }
            NestedFluffyTerm::Hollow(term) => todo!(),
        }
    }
}

impl From<EtherealMethodSignature> for MethodFluffySignature {
    fn from(value: EtherealMethodSignature) -> Self {
        todo!()
    }
}
