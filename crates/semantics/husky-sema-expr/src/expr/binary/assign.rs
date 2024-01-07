use super::*;

impl<'a> SemaExprEngine<'a> {
    pub(super) fn calc_binary_assign_expr_ty(
        &mut self,
        expr_idx: SynExprIdx,
        lopd: SynExprIdx,
        ropd: SynExprIdx,
    ) -> (
        SemaExprIdx,
        SemaBinaryOpr,
        SemaExprIdx,
        SemaExprDataResult<SemaBinaryOprDynamicDispatch>,
        SemaExprTypeResult<FluffyTerm>,
    ) {
        // self
        //     .fluffy_term_region
        //     .new_implicit_symbol(expr_idx, ImplicitSymbolVariant::ExprEvalLifetime);
        let (lopd_sema_expr_idx, lopd_ty) =
            self.build_sema_expr_with_outcome(lopd, ExpectAnyOriginal);
        let ropd_sema_expr_idx = match lopd_ty {
            Some(_) => todo!(),
            None => self.build_sema_expr(ropd, ExpectAnyDerived),
        };
        (
            lopd_sema_expr_idx,
            SemaBinaryOpr::Assign,
            ropd_sema_expr_idx,
            Ok(SemaBinaryOprDynamicDispatch::builtin()),
            Ok(self.term_menu().unit_ty_ontology().into()),
        )
    }
}
