use super::*;

pub(super) fn trai_for_ty_method_ty(
    db: &dyn EtherealTypeDb,
    term: EtherealTerm,
    ident: Ident,
    available_traits: &[TraitPath],
) -> TypeResult<Option<(TypeMethodDisambiguation, TypeResult<EtherealTerm>)>> {
    todo!()
}

#[derive(Debug, PartialEq, Eq)]
pub struct TraitForTypeMethodDisambiguation {
    indirections: SmallVec<[MethodIndirection; 2]>,
    ty_path: TypePath,
}
