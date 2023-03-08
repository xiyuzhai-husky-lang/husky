use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_new_list_expr_ty(
        &mut self,
        expr_idx: ExprIdx,
        items: ExprIdxRange,
        local_term_region: &mut LocalTermRegion,
    ) -> Result<LocalTerm, ExprTypeError> {
        let element_ty: LocalTerm = local_term_region
            .new_implicit_symbol(expr_idx, ImplicitSymbolVariant::ImplicitType)
            .into();
        for item in items {
            self.infer_new_expr_ty_discarded(
                item,
                ExpectImplicitlyConvertible {
                    destination: element_ty,
                },
                local_term_region,
            );
        }
        Ok(local_term_region
            .intern_unresolved_term(
                expr_idx,
                UnresolvedTerm::TypeApplication {
                    ty_path: self.entity_path_menu.list_ty_path(),
                    arguments: vec![element_ty],
                },
            )
            .into())
    }
}
