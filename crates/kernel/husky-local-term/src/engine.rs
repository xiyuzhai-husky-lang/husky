use crate::*;

pub trait LocalTermEngine<'a> {
    fn db(&self) -> &'a dyn TermDb;
    fn unresolved_terms(&self) -> &UnresolvedTerms;

    fn new_ty_ontology_application(
        &mut self,
        src_expr_idx: ExprIdx,
        path: TypePath,
        arguments: SmallVec<[LocalTerm; 2]>,
    ) -> LocalTerm {
        todo!()
    }
}
