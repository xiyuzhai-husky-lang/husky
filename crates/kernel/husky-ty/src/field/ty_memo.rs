use super::*;

pub(super) fn ty_memo_ty(
    db: &dyn TypeDb,
    term: Term,
    ident: Ident,
) -> TypeResult<Option<(TypeMemoDisambiguation, TypeResult<Term>)>> {
    todo!()
}

#[derive(Debug, PartialEq, Eq)]
pub struct TypeMemoDisambiguation {}
