use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_application(
        &mut self,
        function: ExprIdx,
        argument: ExprIdx,
    ) -> Result<LocalTerm, ExprTypeError> {
        let function_expr = &self[function];
        match function_expr {
            Expr::NewBoxList {
                caller: None,
                lbox_token_idx,
                items,
                rbox_token_idx,
            } => {
                match items.len() {
                    0 => {
                        let argument_ty = self.infer_new_expr_ty(argument, ExpectType);
                        // check this is type
                        argument_ty
                            .ok_or(DerivedExprTypeError::ApplicationArgumentTypeNotInferred.into())
                    }
                    1 => {
                        let arg0_ty = self.infer_new_expr_ty(items.start(), ExpectType);
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
            Expr::BoxColon {
                caller,
                lbox_token_idx,
                colon_token_idx,
                rbox_token,
            } => todo!(),
            _ => {
                let function_ty = self.infer_new_expr_ty(function, ExpectType);
                todo!()
            }
        }
    }
}
