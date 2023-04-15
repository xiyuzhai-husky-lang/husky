mod dyn_trai;
mod trai_for_ty;
mod ty;

pub use self::trai_for_ty::*;
pub use self::ty::*;

use super::*;
use husky_word::Ident;

#[derive(Debug, PartialEq, Eq)]
#[enum_class::from_variants]
pub enum FluffyMethodDisambiguation {
    Type(FluffyTypeMethodDisambiguation),
    TraitForType(FluffyTraitForTypeMethodDisambiguation),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FluffyMethodIndirection {}

#[derive(Debug, PartialEq, Eq)]
pub enum FluffyMethodDisambiguationVariant {
    Type,
    Trait {
        trai_path: TraitPath,
        trai: FluffyTerm,
    },
}

impl FluffyTerm {
    pub fn method_ty(
        self,
        engine: &mut impl FluffyTermEngine,
        ident: Ident,
        available_traits: &[TraitPath],
    ) -> FluffyTypeResult<(FluffyMethodDisambiguation, FluffyTypeResult<FluffyTerm>)> {
        if let Some((disambiguation, ty_result)) = self.ty_method_ty(engine, ident)? {
            return Ok((disambiguation.into(), ty_result));
        }
        if let Some((disambiguation, ty_result)) =
            self.trai_for_ty_method_ty(ident, available_traits)?
        {
            return Ok((disambiguation.into(), ty_result));
        }
        Err(OriginalFluffyTypeError::NoSuchMethod.into())
    }
}
