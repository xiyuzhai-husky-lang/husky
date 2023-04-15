mod dyn_trai;
mod trai_for_ty;
mod ty;

pub use self::dyn_trai::*;
pub use self::trai_for_ty::*;
pub use self::ty::*;

use super::*;

impl Term {
    pub fn method_ty(
        self,
        db: &dyn TermDb,
        ident: Ident,
        available_traits: &[TraitPath],
    ) -> TermResult<(MethodDisambiguation, TermResult<Term>)> {
        if let Some((disambiguation, ty_result)) = self.ty_method_ty(db, ident)? {
            return Ok((disambiguation.into(), ty_result));
        }
        if let Some((disambiguation, ty_result)) =
            self.trai_for_ty_method_ty(db, ident, available_traits)?
        {
            return Ok((disambiguation.into(), ty_result));
        }
        Err(TermError::NoSuchMethod.into())
    }
}

#[derive(Debug, PartialEq, Eq)]
#[enum_class::from_variants]
pub enum MethodDisambiguation {
    Type(TypeMethodDisambiguation),
    TraitForType(TraitForTypeMethodDisambiguation),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MethodIndirection {}
