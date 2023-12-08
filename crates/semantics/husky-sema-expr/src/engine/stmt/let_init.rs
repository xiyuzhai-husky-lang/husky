use husky_regional_token::{EqRegionalToken, LetRegionalToken};

use super::*;

impl<'a> SemaExprEngine<'a> {
    pub(super) fn calc_let_stmt(
        &mut self,
        let_token: LetRegionalToken,
        let_pattern_syn_obelisk: &'a SynExprResult<LetPatternSynSyndicate>,
        eq_token: &SynExprResult<EqRegionalToken>,
        initial_value: SynExprIdx,
    ) -> (
        SemaExprDataResult<SemaStmtData>,
        SemaExprTypeResult<FluffyTerm>,
    ) {
        let (let_pattern_sema_obelisk, annotated_pattern_ty) =
            match let_pattern_syn_obelisk.as_ref() {
                Ok(let_pattern_syn_obelisk) => {
                    let let_pattern_sema_obelisk =
                        match self.build_let_pattern_sema_obelisk(let_pattern_syn_obelisk) {
                            Ok(let_pattern_sema_obelisk) => let_pattern_sema_obelisk,
                            Err(_) => todo!(),
                        };
                    (
                        let_pattern_sema_obelisk,
                        match let_pattern_sema_obelisk.ty_sema_expr_idx() {
                            Some(ty_sema_expr_idx) => self.infer_expr_term(ty_sema_expr_idx),
                            None => None,
                        },
                    )
                }
                Err(e) => {
                    p!(e.debug(self.db()));
                    todo!()
                }
            };
        let contract = self.syn_expr_region_data.pattern_contract(
            let_pattern_syn_obelisk
                .as_ref()
                .expect("must be okay")
                .syn_pattern_root()
                .syn_pattern_expr_idx(),
        );
        let ((initial_value_sema_expr_idx, pattern_ty), coersion) = match annotated_pattern_ty {
            Some(pattern_ty) => {
                let (initial_value_sema_expr_idx, coersion) = self.build_sema_expr_with_outcome(
                    initial_value,
                    ExpectCoersion::new(contract, pattern_ty),
                );
                ((initial_value_sema_expr_idx, Some(pattern_ty)), coersion)
            }
            None => {
                (
                    self.build_sema_expr_with_ty(
                        initial_value,
                        // ad hoc
                        ExpectAnyOriginal,
                    ),
                    None,
                )
            }
        };
        let ty_result = match pattern_ty {
            Some(ty) if ty == self.term_menu.never().into() => Ok(self.term_menu.never().into()),
            Some(ty) => {
                match let_pattern_syn_obelisk {
                    Ok(let_variables_pattern) => self.infer_variable_pattern_root_and_symbols_ty(
                        let_variables_pattern.syn_pattern_root(),
                        ty,
                        let_variables_pattern.variables(),
                    ),
                    Err(_) => todo!(),
                };
                Ok(self.term_menu.unit_ty_ontology().into())
            }
            None => Ok(self.term_menu.unit_ty_ontology().into()),
        };
        let eq_token = match eq_token {
            Ok(eq_token) => *eq_token,
            Err(_) => todo!(),
        };
        (
            Ok(SemaStmtData::Let {
                let_token,
                let_pattern_sema_obelisk,
                contract,
                eq_token,
                initial_value_sema_expr_idx,
                coersion,
            }),
            ty_result,
        )
    }
}
