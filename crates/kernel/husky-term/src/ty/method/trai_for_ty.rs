use super::*;

impl Term {
    pub(super) fn trai_for_ty_method_ty(
        self,
        db: &dyn TermDb,
        ident: Ident,
        available_traits: &[TraitPath],
    ) -> TermResult<Option<(TypeMethodDisambiguation, TermResult<Term>)>> {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TraitForTypeMethodDisambiguation {
    indirections: SmallVec<[MethodIndirection; 2]>,
    ty_path: TypePath,
}
