mod ethereal;
mod hollow;
mod solid;

pub(crate) use self::ethereal::*;
pub(crate) use self::hollow::*;
pub(crate) use self::solid::*;

use super::*;
use husky_word::Ident;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
pub struct FluffyFieldDisambiguation {
    indirections: SmallVec<[FluffyFieldIndirection; 2]>,
    ty_path: TypePath,
    signature: FieldSignature<FluffyTerm>,
}

impl FluffyFieldDisambiguation {
    fn merge(&self, mut indirections: SmallVec<[FluffyFieldIndirection; 2]>) -> Self {
        indirections.extend(self.indirections.iter().copied());
        Self {
            indirections,
            ty_path: self.ty_path,
            signature: self.signature,
        }
    }
}

impl FluffyTerm {
    /// returns None if no such field
    pub fn field_disambiguation(
        self,
        engine: &mut impl FluffyTermEngine,
        ident: Ident,
        available_traits: &[TraitPath],
    ) -> FluffyTermMaybeResult<FluffyFieldDisambiguation> {
        self.field_disambiguation_aux(engine, ident, available_traits, smallvec![])
    }

    fn field_disambiguation_aux(
        self,
        engine: &mut impl FluffyTermEngine,
        ident: Ident,
        available_traits: &[TraitPath],
        mut indirections: SmallVec<[FluffyFieldIndirection; 2]>,
    ) -> FluffyTermMaybeResult<FluffyFieldDisambiguation> {
        match self.nested() {
            NestedFluffyTerm::Ethereal(term) => JustOk(
                ethereal_ty_field_disambiguation(engine.db(), term, ident)?.merge(indirections),
            ),
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
    Leash,
}
