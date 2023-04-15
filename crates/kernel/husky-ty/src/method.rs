mod dyn_trai;
mod trai_for_ty;
mod ty;

pub use self::dyn_trai::*;
pub use self::trai_for_ty::*;
pub use self::ty::*;

use crate::*;

pub trait HasMethodType: Copy {
    fn method_ty(
        self,
        db: &dyn TypeDb,
        ident: Ident,
        available_traits: &[TraitPath],
    ) -> TypeResult<(MethodDisambiguation, TypeResult<Term>)>;
}

impl HasMethodType for Term {
    fn method_ty(
        self,
        db: &dyn TypeDb,
        ident: Ident,
        available_traits: &[TraitPath],
    ) -> TypeResult<(MethodDisambiguation, TypeResult<Term>)> {
        if let Some((disambiguation, ty_result)) = ty_method_ty(db, self, ident)? {
            return Ok((disambiguation.into(), ty_result));
        }
        if let Some((disambiguation, ty_result)) =
            trai_for_ty_method_ty(db, self, ident, available_traits)?
        {
            return Ok((disambiguation.into(), ty_result));
        }
        Err(OriginalTypeError::NoSuchMethod.into())
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
