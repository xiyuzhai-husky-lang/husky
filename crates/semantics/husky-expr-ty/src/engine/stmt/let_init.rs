use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_let_stmt(
        &mut self,
        let_variable_decls: &SynExprResult<LetVariableObelisk>,
        initial_value: SynExprIdx,
    ) -> Option<FluffyTerm> {
        let annotated_pattern_ty = match let_variable_decls {
            Ok(pattern) => match pattern.ty() {
                Some(ty) => {
                    self.infer_new_expr_ty_discarded(
                        ty,
                        ExpectEqsCategory::new_expect_eqs_ty_kind(),
                    );
                    self.infer_expr_term(ty)
                }
                None => None,
            },
            Err(e) => {
                p!(e.debug(self.db()));
                todo!()
            }
        };
        let pattern_ty = match annotated_pattern_ty {
            Some(pattern_ty) => {
                let contract = self.expr_region_data.pattern_contract(
                    let_variable_decls
                        .as_ref()
                        .expect("must be okay")
                        .pattern_expr_idx(),
                );
                self.infer_new_expr_ty_discarded(
                    initial_value,
                    ExpectCoersion::new(contract, pattern_ty),
                );
                Some(pattern_ty)
            }
            None => {
                self.infer_new_expr_ty(
                    initial_value,
                    // ad hoc
                    ExpectAnyOriginal,
                )
            }
        };
        match pattern_ty {
            Some(ty) if ty == self.term_menu.never().into() => Some(self.term_menu.never().into()),
            Some(ty) => {
                match let_variable_decls {
                    Ok(let_variables_pattern) => self.infer_pattern_and_symbols_ty(
                        let_variables_pattern.pattern_expr_idx(),
                        ty,
                        let_variables_pattern.variables(),
                    ),
                    Err(_) => todo!(),
                };
                Some(self.term_menu.unit_ty_ontology().into())
            }
            None => Some(self.term_menu.unit_ty_ontology().into()),
        }
    }
}
