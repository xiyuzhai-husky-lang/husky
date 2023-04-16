mod regular;
mod ty_memo;

pub use self::regular::*;
pub use self::ty_memo::*;

use super::*;
use husky_word::Ident;

impl FluffyTerm {
    pub fn field_ty(
        self,
        engine: &mut impl FluffyTermEngine,
        ident: Ident,
        available_traits: &[TraitPath],
    ) -> FluffyTypeResult<(FluffyFieldDisambiguation, FluffyTypeResult<FluffyTerm>)> {
        let Some(card) = self.field_card(engine, ident, available_traits)? else {
            Err(OriginalFluffyTypeError::NoSuchField)?
        };
        // p!(card.debug(engine.db()));
        todo!()
    }

    pub fn field_ty_aux(
        self,
        engine: &mut impl FluffyTermEngine,
        ident: Ident,
        available_traits: &[TraitPath],
        indirections: &mut SmallVec<[FluffyIndirection; 2]>,
    ) -> FluffyTypeResult<(FluffyFieldDisambiguation, FluffyTypeResult<FluffyTerm>)> {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq)]
#[enum_class::from_variants]
pub enum FluffyFieldDisambiguation {
    Regular(FluffyRegularFieldDisambiguation),
    TypeMemo(FluffyTypeMemoFieldDisambiguation),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FluffyFieldIndirection {}
