use super::*;

pub(super) fn regular_field_ty(
    db: &dyn TypeDb,
    term: EtherealTerm,
    ident: Ident,
) -> TypeResult<Option<(RegularFieldDisambiguation, TypeResult<EtherealTerm>)>> {
    todo!()
}

#[derive(Debug, PartialEq, Eq)]
pub struct RegularFieldDisambiguation {}
