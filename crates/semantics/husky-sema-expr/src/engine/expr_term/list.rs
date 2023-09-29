use super::*;

impl<'a> SemaExprEngine<'a> {
    pub(super) fn calc_new_list_expr_term(
        &self,
        expr_idx: SemaExprIdx,
        items: &[SemaCommaListItem],
    ) -> SemaExprTermResult<FluffyTerm> {
        todo!()
        // match self
        //     .expr_ty_info_variant(expr_idx)
        //     .map_err(|_| DerivedExprTermError::Todo)?
        //     .list_expr_disambiguation()
        //     .expect("seriously?")
        // {
        //     ListExprDisambiguation::NewList => todo!(),
        //     ListExprDisambiguation::ListFunctor => {
        //         assert_eq!(items.len(), 0);
        //         Ok(self.term_menu.list_ty_ontology().into())
        //     }
        //     ListExprDisambiguation::ArrayFunctor => todo!(),
        // }
    }
}
