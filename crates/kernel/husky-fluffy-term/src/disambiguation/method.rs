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
use husky_word::Ident;

impl FluffyTerm {
    pub fn method_ty(
        self,
        engine: &mut impl FluffyTermEngine,
        ident: Ident,
        available_traits: &[TraitPath],
    ) -> FluffyTermResult<(FluffyMethodDisambiguation, FluffyTermResult<FluffyTerm>)> {
        // let Some(card) = self.method_card(engine, ident, available_traits)? else {
        //     Err(OriginalFluffyTermError::NoSuchMethod)?
        // };
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq)]
#[enum_class::from_variants]
pub enum FluffyMethodDisambiguation {
    Type(FluffyTypeMethodDisambiguation),
    TraitForType(FluffyTraitForTypeMethodDisambiguation),
}

#[derive(Debug, PartialEq, Eq)]
pub struct FluffyTypeMethodDisambiguation {
    indirections: SmallVec<[FluffyMethodIndirection; 2]>,
    ty_path: TypePath,
}

#[derive(Debug, PartialEq, Eq)]
pub struct FluffyTraitForTypeMethodDisambiguation {
    indirections: SmallVec<[FluffyMethodIndirection; 2]>,
    ty_path: TypePath,
    trai_path: TraitPath,
    trai: FluffyTerm,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FluffyMethodIndirection {}
