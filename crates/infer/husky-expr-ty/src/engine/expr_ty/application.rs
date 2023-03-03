use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_application_expr_ty(
        &mut self,
        function: ExprIdx,
        argument: ExprIdx,
        local_term_region: &mut LocalTermRegion,
    ) -> Result<LocalTerm, ExprTypeError> {
        let function_expr = &self[function];
        match function_expr {
            Expr::BoxList {
                lbox_token_idx,
                items,
                rbox_token_idx,
            } => {
                match items.len() {
                    0 => {
                        let argument_ty = self.infer_new_expr_ty(
                            argument,
                            /* ad hoc */ ExpectAnyOriginal,
                            local_term_region,
                        );
                        // check this is type
                        argument_ty
                            .ok_or(DerivedExprTypeError::ApplicationArgumentTypeNotInferred.into())
                    }
                    1 => {
                        let arg0_ty = self.infer_new_expr_ty(
                            items.start(),
                            /* ad hoc */ ExpectAnyOriginal,
                            local_term_region,
                        );
                        match arg0_ty {
                            Some(_) => todo!(),
                            None => {
                                Err(DerivedExprTypeError::BoxListApplicationFirstArgumentError
                                    .into())
                            }
                        }
                    }
                    n => {
                        todo!()
                    }
                }
            }
            Expr::BoxColonList {
                lbox_token_idx,
                colon_token_idx,
                ..
            } => todo!(),
            _ => {
                let function_ty = self.infer_new_expr_ty(
                    function,
                    /* ad hoc */ ExpectAnyOriginal,
                    local_term_region,
                );
                Err(OriginalExprTypeError::TodoBoxColon.into())
            }
        }
    }
}
