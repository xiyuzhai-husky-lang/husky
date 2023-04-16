mod regular;
mod ty_memo;

pub use self::regular::*;
pub use self::ty_memo::*;

use super::*;

pub trait HasFieldType: Copy {
    fn field_ty(
        self,
        db: &dyn TypeDb,
        ident: Ident,
        available_traits: &[TraitPath],
    ) -> TypeResult<(FieldDisambiguation, TypeResult<EtherealTerm>)> {
        if let Some((disambiguation, ty_result)) = self.regular_field_ty(db, ident)? {
            return Ok((disambiguation.into(), ty_result));
        }
        if let Some((disambiguation, ty_result)) = self.ty_memo_ty(db, ident)? {
            return Ok((disambiguation.into(), ty_result));
        }
        // todo: trait for ty memos
        // todo: dyn trait memos
        Err(OriginalTypeError::NoSuchMethod.into())
    }

    fn regular_field_ty(
        self,
        db: &dyn TypeDb,
        ident: Ident,
    ) -> TypeResult<Option<(RegularFieldDisambiguation, TypeResult<EtherealTerm>)>>;
    fn ty_memo_ty(
        self,
        db: &dyn TypeDb,
        ident: Ident,
    ) -> TypeResult<Option<(TypeMemoDisambiguation, TypeResult<EtherealTerm>)>>;
}

impl HasFieldType for EtherealTerm {
    fn regular_field_ty(
        self,
        db: &dyn TypeDb,
        ident: Ident,
    ) -> TypeResult<Option<(RegularFieldDisambiguation, TypeResult<EtherealTerm>)>> {
        regular_field_ty(db, self, ident)
    }

    fn ty_memo_ty(
        self,
        db: &dyn TypeDb,
        ident: Ident,
    ) -> TypeResult<Option<(TypeMemoDisambiguation, TypeResult<EtherealTerm>)>> {
        ty_memo_ty(db, self, ident)
    }
}

#[derive(Debug, PartialEq, Eq)]
#[enum_class::from_variants]
pub enum FieldDisambiguation {
    RegularField(RegularFieldDisambiguation),
    TypeMemo(TypeMemoDisambiguation),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FieldModifier {
    Pure,
    Mut,
    Const,
    Leashed,
}
