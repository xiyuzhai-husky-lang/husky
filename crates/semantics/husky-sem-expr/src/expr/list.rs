use super::*;

impl<'a> SemExprBuilder<'a> {
    pub(super) fn calc_new_list_expr_term(
        &self,
        expr_idx: SemExprIdx,
        items: &[SemCommaListItem],
    ) -> SemExprTermResult<FlyTerm> {
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
