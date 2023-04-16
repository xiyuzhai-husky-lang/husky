use super::*;

pub(super) fn regular_field_ty(
    db: &dyn TypeDb,
    term: Term,
    ident: Ident,
) -> TypeResult<Option<(RegularFieldDisambiguation, TypeResult<Term>)>> {
    todo!()
}

#[derive(Debug, PartialEq, Eq)]
pub struct RegularFieldDisambiguation {}
