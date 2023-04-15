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
        if let Some((disambiguation, ty_result)) = self.regular_field_ty(engine, ident)? {
            return Ok((disambiguation.into(), ty_result));
        }
        if let Some((disambiguation, ty_result)) = self.ty_memo_ty(engine, ident)? {
            return Ok((disambiguation.into(), ty_result));
        }
        // todo: trait for ty memos
        // todo: dyn trait memos
        Err(OriginalFluffyTypeError::NoSuchMethod.into())
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
