mod dyn_trai;
mod trai_for_ty;
mod ty;

pub use self::trai_for_ty::*;
pub use self::ty::*;

use super::*;
use husky_word::Ident;

impl FluffyTerm {
    pub fn method_ty(
        self,
        engine: &mut impl FluffyTermEngine,
        ident: Ident,
        available_traits: &[TraitPath],
    ) -> FluffyTypeResult<(FluffyMethodTypeInfo, FluffyTypeResult<FluffyTerm>)> {
        let Some(card) = self.method_card(engine, ident, available_traits)? else {
            Err(OriginalFluffyTypeError::NoSuchMethod)?
        };
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq)]
#[enum_class::from_variants]
pub enum FluffyMethodTypeInfo {
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
