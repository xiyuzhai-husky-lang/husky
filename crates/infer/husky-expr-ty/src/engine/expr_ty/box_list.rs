use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_new_box_list(
        &mut self,
        expr_idx: ExprIdx,
        items: ExprIdxRange,
    ) -> Result<LocalTerm, ExprTypeError> {
        let element_ty = self.new_implicit_symbol(expr_idx, ImplicitSymbolVariant::ImplicitType);
        for item in items {
            self.infer_new_expr_ty(
                item,
                ExpectImplicitlyConvertible {
                    destination: element_ty,
                },
            );
        }
        Ok(self
            .intern_unresolved_term(
                expr_idx,
                UnresolvedTerm::TypeApplication {
                    ty: self.entity_path_menu.list_ty(),
                    arguments: vec![element_ty],
                },
            )
            .into())
    }
}
