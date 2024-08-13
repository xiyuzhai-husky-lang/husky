use super::*;
use husky_hir_lazy_expr::helpers::debug::HirLazyExprDebugger;

impl<Devsoul: IsDevsoul> DevRuntime<Devsoul> {
    /// used for debugging purposes
    ///
    /// todo: make this safer by considering var deps, i.e., only eval those whose var deps are contained in ki_repr's var deps
    pub(super) fn eval_all_in_expansion(
        &self,
        ki_repr: KiRepr,
    ) -> Vec<(String, DevsoulKiControlFlow<Devsoul>)> {
        let db = self.db();
        let expansion = ki_repr.expansion(db).expect("should be expandible");
        let item_path = expansion.path(db);
        let debugger = HirLazyExprDebugger::new_body(item_path, db).unwrap();
        use ::husky_print_utils::p;
        use ::salsa::DebugWithDb;
        p!(item_path.debug(db));
        expansion
            .hir_lazy_stmt_ki_repr_map(db)
            .iter()
            .map(|(stmt, &ki_repr)| {
                let stmt = debugger.stmt_text(stmt).to_string();
                let ki_repr = self.eval_ki_repr(ki_repr);
                (stmt, ki_repr)
            })
            .chain(
                expansion
                    .hir_lazy_expr_ki_repr_map(db)
                    .iter()
                    .map(|(expr, &ki_repr)| {
                        let expr = debugger.expr_text(expr).to_string();
                        let ki_repr = self.eval_ki_repr(ki_repr);
                        (expr, ki_repr)
                    }),
            )
            .collect()
    }
}
