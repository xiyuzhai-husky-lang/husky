mod let_init;

use super::*;

impl<'a> SemaExprEngine<'a> {
    pub(crate) fn build_sema_branch<Expectation: ExpectFluffyTerm>(
        &mut self,
        stmts: SynStmtIdxRange,
        merger: &mut BranchTypeMerger<Expectation>,
    ) -> SemaStmtIdxRange {
        let (stmts, block_ty) =
            self.build_sema_block_with_its_ty_returned(stmts, merger.expr_expectation().clone());
        merger.add(self, block_ty);
        stmts
    }

    pub(crate) fn build_sema_block(
        &mut self,
        stmts: SynStmtIdxRange,
        block_ty_expectation: impl ExpectFluffyTerm,
    ) -> SemaStmtIdxRange {
        let mut batch = SemaStmtBatch::default();
        for stmt in stmts.start()..(stmts.end() - 1) {
            batch.add(self.infer_new_nonlast_stmt(stmt));
        }
        batch.add(self.build_sema_stmt(stmts.end() - 1, block_ty_expectation));
        self.sema_stmt_arena.alloc_batch(batch)
    }

    pub(crate) fn build_sema_block_with_its_ty_returned(
        &mut self,
        stmts: SynStmtIdxRange,
        block_ty_expectation: impl ExpectFluffyTerm,
    ) -> (SemaStmtIdxRange, Option<FluffyTerm>) {
        let mut batch = SemaStmtBatch::default();
        for stmt in stmts.start()..(stmts.end() - 1) {
            batch.add(self.infer_new_nonlast_stmt(stmt));
        }
        let (data_result, ty_result) = self.build_sema_stmt(stmts.end() - 1, block_ty_expectation);
        let ty = ty_result.as_ref().ok().copied();
        batch.add((data_result, ty_result));
        (self.sema_stmt_arena.alloc_batch(batch), ty)
    }

    fn infer_new_nonlast_stmt(
        &mut self,
        stmt_idx: SynStmtIdx,
    ) -> (
        SemaExprDataResult<SemaStmtData>,
        SemaExprTypeResult<FluffyTerm>,
    ) {
        let expect_unit = self.expect_unit();
        self.build_sema_stmt(stmt_idx, expect_unit)
    }

    fn build_sema_stmt(
        &mut self,
        stmt_idx: SynStmtIdx,
        stmt_ty_expectation: impl ExpectFluffyTerm,
    ) -> (
        SemaExprDataResult<SemaStmtData>,
        SemaExprTypeResult<FluffyTerm>,
    ) {
        match self.syn_expr_region_data[stmt_idx] {
            SynStmtData::Let {
                let_token,
                ref let_variables_pattern,
                ref assign_token,
                initial_value,
            } => self.calc_let_stmt(
                let_token,
                let_variables_pattern,
                assign_token,
                initial_value,
            ),
            SynStmtData::Return {
                return_token,
                result,
            } => {
                let result = match self.return_ty {
                    Some(return_ty) => {
                        self.build_sema_expr(result, ExpectCoersion::new_move(return_ty.into()))
                    }
                    None => self.build_sema_expr(result, ExpectAnyDerived),
                };
                (
                    Ok(SemaStmtData::Return {
                        return_token,
                        result,
                    }),
                    Ok(self.term_menu.never().into()),
                )
            }
            SynStmtData::Require {
                require_token,
                condition,
            } => {
                let condition = self.build_sema_condition(condition);
                (
                    Ok(SemaStmtData::Require {
                        require_token,
                        condition,
                    }),
                    Ok(self.term_menu.unit_ty_ontology().into()),
                )
            }
            SynStmtData::Assert {
                assert_token,
                condition,
            } => {
                let condition = self.build_sema_condition(condition);
                (
                    Ok(SemaStmtData::Assert {
                        assert_token,
                        condition,
                    }),
                    Ok(self.term_menu.unit_ty_ontology().into()),
                )
            }
            SynStmtData::Break { break_token } => (
                Ok(SemaStmtData::Break { break_token }),
                Ok(self.term_menu.never().into()),
            ),
            SynStmtData::Eval {
                expr_idx,
                eol_semicolon,
            } => {
                let (sema_expr_idx, ty) = match eol_semicolon {
                    Ok(None) => {
                        self.build_sema_expr_with_its_ty_returned(expr_idx, stmt_ty_expectation)
                    }
                    Ok(Some(_)) => {
                        let (sema_expr_idx, expr_ty) =
                            self.build_sema_expr_with_its_ty_returned(expr_idx, ExpectAnyOriginal);
                        let ty_result = match expr_ty {
                            Some(ty) => match ty.base_resolved(self) {
                                FluffyTermBase::Ethereal(ty) if ty == self.term_menu.never() => {
                                    Some(self.term_menu.never().into())
                                }
                                _ => Some(self.term_menu.unit_ty_ontology().into()),
                            },
                            None => None,
                        };
                        (sema_expr_idx, ty_result)
                    }
                    Err(_) => self.build_sema_expr_with_its_ty_returned(expr_idx, ExpectAnyDerived),
                };
                (
                    Ok(SemaStmtData::Eval {
                        sema_expr_idx,
                        eol_semicolon,
                    }),
                    ty.ok_or(DerivedSemaExprTypeError::EvalExprTypeNotInferred.into()),
                )
            }
            SynStmtData::ForBetween {
                for_token,
                ref particulars,
                for_loop_var_symbol_idx,
                ref block,
                ref eol_colon,
            } => {
                let expr_expectation = self.expect_unit();
                let Ok(particulars) =
                    self.build_sema_for_between_particulars(particulars, for_loop_var_symbol_idx)
                else {
                    todo!()
                };
                let block = self.build_sema_block(*block, expr_expectation);
                let &Ok(eol_colon) = eol_colon else { todo!() };
                (
                    Ok(SemaStmtData::ForBetween {
                        for_token,
                        particulars,
                        for_loop_var_symbol_idx,
                        eol_colon,
                        block,
                    }),
                    Ok(self.term_menu.unit_ty_ontology().into()),
                )
            }
            SynStmtData::ForIn {
                ref condition,
                block,
                ..
            } => todo!(),
            SynStmtData::ForExt {
                forext_token,
                ref particulars,
                block,
                ref eol_colon,
            } => {
                let (forext_loop_var_sema_expr_idx, forext_loop_var_ty) = self
                    .build_sema_expr_with_its_ty_returned(
                        particulars.forext_loop_var_expr_idx,
                        ExpectAnyOriginal,
                    );
                let Some(forext_loop_var_ty) = forext_loop_var_ty else {
                    todo!()
                };
                let particulars = self.build_sema_forext_particulars(
                    particulars,
                    forext_loop_var_sema_expr_idx,
                    forext_loop_var_ty,
                );
                let block_ty_expectation = self.expect_unit();
                let block = self.build_sema_block(block, block_ty_expectation);
                let &Ok(eol_colon) = eol_colon else { todo!() };
                (
                    Ok(SemaStmtData::Forext {
                        forext_token,
                        particulars,
                        eol_colon,
                        block,
                    }),
                    Ok(self.term_menu.unit_ty_ontology().into()),
                )
            }
            SynStmtData::While {
                while_token,
                ref condition,
                block,
                ref eol_colon,
            } => {
                let condition = match condition {
                    Ok(condition) => self.build_sema_condition(*condition),
                    Err(_) => todo!(),
                };
                let &Ok(eol_colon) = eol_colon else { todo!() };
                let expect_unit = self.expect_unit();
                let block = self.build_sema_block(block, expect_unit);
                (
                    Ok(SemaStmtData::While {
                        while_token,
                        condition,
                        eol_colon,
                        block,
                    }),
                    Ok(self.term_menu.unit_ty_ontology().into()),
                )
            }
            SynStmtData::DoWhile {
                do_token,
                while_token,
                ref condition,
                block,
                ref eol_colon,
            } => {
                let condition = match condition {
                    Ok(condition) => self.build_sema_condition(*condition),
                    Err(_) => todo!(),
                };
                let &Ok(eol_colon) = eol_colon else { todo!() };
                let expect_unit = self.expect_unit();
                let block = self.build_sema_block(block, expect_unit);
                (
                    Ok(SemaStmtData::DoWhile {
                        do_token,
                        while_token,
                        condition,
                        eol_colon,
                        block,
                    }),
                    Ok(self.term_menu.unit_ty_ontology().into()),
                )
            }
            SynStmtData::IfElse {
                ref if_branch,
                ref elif_branches,
                ref else_branch,
            } => self.build_if_else_stmt(
                if_branch,
                elif_branches,
                else_branch.as_ref(),
                stmt_ty_expectation,
            ),
            SynStmtData::Match {
                match_token,
                ref match_expr,
                ref eol_with_token,
                ref case_branches,
            } => self.build_match_stmt(
                match_token,
                match_expr,
                eol_with_token,
                case_branches,
                stmt_ty_expectation,
            ),
        }
    }

    pub(crate) fn build_sema_condition(&mut self, syn_expr_idx: SynExprIdx) -> SemaCondition {
        let sema_expr_idx = self.build_sema_expr(syn_expr_idx, ExpectConditionType);
        match *sema_expr_idx.data(self.sema_expr_arena.arena_ref()) {
            SemaExprData::Be {
                src,
                be_regional_token_idx,
                target,
            } => SemaCondition::Be {
                src,
                be_regional_token_idx,
                target,
            },
            _ => SemaCondition::Other(sema_expr_idx),
        }
    }
}
