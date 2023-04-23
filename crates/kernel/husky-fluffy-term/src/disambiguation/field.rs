mod ethereal;
mod hollow;
mod solid;

pub(crate) use self::ethereal::*;
pub(crate) use self::hollow::*;
pub(crate) use self::solid::*;

use super::*;
use husky_word::Ident;

#[derive(Debug, PartialEq, Eq)]
pub struct FluffyFieldDisambiguation {
    indirections: SmallVec<[FluffyFieldIndirection; 2]>,
    ty_path: TypePath,
    signature: FieldSignature<FluffyTerm>,
}

impl FluffyTerm {
    /// returns None if no such field
    pub fn field_disambiguation(
        self,
        engine: &mut impl FluffyTermEngine,
        ident: Ident,
        available_traits: &[TraitPath],
    ) -> FluffyTermResult<Option<FluffyFieldDisambiguation>> {
        self.field_disambiguation_aux(engine, ident, available_traits, smallvec![])
    }

    fn field_disambiguation_aux(
        self,
        engine: &mut impl FluffyTermEngine,
        ident: Ident,
        available_traits: &[TraitPath],
        indirections: SmallVec<[FluffyFieldIndirection; 2]>,
    ) -> FluffyTermResult<Option<FluffyFieldDisambiguation>> {
        match self.nested() {
            NestedFluffyTerm::Ethereal(term) => todo!(),
            NestedFluffyTerm::Solid(term) => {
                term.field_disambiguation_aux(engine, ident, available_traits, indirections)
            }
            NestedFluffyTerm::Hollow(term) => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FluffyFieldIndirection {
    Place(Place),
}
