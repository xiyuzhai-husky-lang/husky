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
        expr_expectation: impl ExpectFluffyTerm,
    ) -> SemaStmtIdxRange {
        let mut batch = SemaStmtBatch::default();
        for stmt in stmts.start()..(stmts.end() - 1) {
            batch.add(self.infer_new_nonlast_stmt(stmt));
        }
        batch.add(self.build_sema_stmt(stmts.end() - 1, expr_expectation));
        self.sema_stmt_arena.alloc_batch(batch)
    }

    pub(crate) fn build_sema_block_with_its_ty_returned(
        &mut self,
        stmts: SynStmtIdxRange,
        expr_expectation: impl ExpectFluffyTerm,
    ) -> (SemaStmtIdxRange, Option<FluffyTerm>) {
        let mut batch = SemaStmtBatch::default();
        for stmt in stmts.start()..(stmts.end() - 1) {
            batch.add(self.infer_new_nonlast_stmt(stmt));
        }
        let (data_result, ty_result) = self.build_sema_stmt(stmts.end() - 1, expr_expectation);
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
        expr_expectation: impl ExpectFluffyTerm,
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
                let condition = self.build_sema_expr(condition, ExpectConditionType);
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
                let condition = self.build_sema_expr(condition, ExpectConditionType);
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
                        self.build_sema_expr_with_its_ty_returned(expr_idx, expr_expectation)
                    }
                    Ok(Some(_)) => {
                        let (sema_expr_idx, expr_ty) =
                            self.build_sema_expr_with_its_ty_returned(expr_idx, ExpectAnyOriginal);
                        let ty = match expr_ty {
                            Some(_) => todo!(),
                            None => todo!(),
                        };
                        todo!("unit or never")
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
                ref particulars,
                frame_var_symbol_idx,
                ref block,
                ..
            } => {
                let mut expected_frame_var_ty: Option<FluffyTerm> = None;
                let Ok(ref range) = particulars.range else {
                    todo!()
                };
                let initial_bound_sema_expr_idx = match range.initial_boundary.bound_expr {
                    Some(bound_expr) => {
                        let (bound_sema_expr_idx, num_ty_outcome) =
                            self.build_sema_expr_with_outcome(bound_expr, ExpectIntType);
                        match num_ty_outcome {
                            Some(num_ty_outcome) => {
                                expected_frame_var_ty = Some(num_ty_outcome.placeless_num_ty())
                            }
                            None => (),
                        };
                        Some(bound_sema_expr_idx)
                    }
                    None => None,
                };
                let final_bound_sema_expr_idx = match range.final_boundary.bound_expr {
                    Some(bound_expr) => match expected_frame_var_ty {
                        Some(expected_frame_var_ty) => Some(self.build_sema_expr(
                            bound_expr,
                            ExpectCoersion::new_pure(self, expected_frame_var_ty),
                        )),
                        None => {
                            let (final_bound_sema_expr_idx, ty) = self
                                .build_sema_expr_with_its_ty_returned(
                                    bound_expr,
                                    ExpectAnyOriginal,
                                );
                            if let Some(ty) = ty {
                                expected_frame_var_ty = Some(ty)
                            }
                            Some(final_bound_sema_expr_idx)
                        }
                    },
                    None => None,
                };
                if let Some(expected_frame_var_ty) = expected_frame_var_ty {
                    let place = Place::ImmutableStackOwned {
                        location: frame_var_symbol_idx
                            .into_local_symbol_idx(self.syn_expr_region_data)
                            .into(),
                    };
                    let frame_var_symbol_ty =
                        SymbolType::new(self, frame_var_symbol_idx, expected_frame_var_ty);
                    self.symbol_tys
                        .insert_new(frame_var_symbol_idx, frame_var_symbol_ty)
                }
                let expr_expectation = self.expect_unit();
                self.build_sema_block_with_its_ty_returned(*block, expr_expectation);
                (todo!(), Ok(self.term_menu.unit_ty_ontology().into()))
            }
            SynStmtData::ForIn {
                ref condition,
                block,
                ..
            } => todo!(),
            SynStmtData::ForExt {
                ref particulars,
                block,
                ..
            } => {
                let (forext_loop_var_sema_expr_idx, forext_loop_var_ty) = self
                    .build_sema_expr_with_its_ty_returned(
                        particulars.forext_loop_var_expr_idx,
                        ExpectIntType,
                    );
                let Some(forext_loop_var_ty) = forext_loop_var_ty else {
                    todo!()
                };
                self.build_sema_expr(
                    particulars.bound_expr,
                    ExpectCoersion::new_pure(self, forext_loop_var_ty),
                );
                let expr_expectation = self.expect_unit();
                let block = self.build_sema_block(block, expr_expectation);
                (
                    Ok(SemaStmtData::Forext {
                        forext_token: todo!(),
                        particulars: todo!(),
                        eol_colon: todo!(),
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
                    Ok(condition) => self.build_sema_expr(*condition, ExpectConditionType),
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
                    Ok(condition) => self.build_sema_expr(*condition, ExpectConditionType),
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
                expr_expectation,
            ),
            SynStmtData::Match { .. } => {
                // todo: match
                // None
                todo!()
            }
        }
    }

    fn build_if_else_stmt(
        &mut self,
        syn_if_branch: &'a SynIfBranch,
        syn_elif_branches: &'a [SynElifBranch],
        syn_else_branch: Option<&'a SynElseBranch>,
        expr_expectation: impl ExpectFluffyTerm,
    ) -> (
        SemaExprDataResult<SemaStmtData>,
        SemaExprTypeResult<FluffyTerm>,
    ) {
        let mut merger = BranchTypeMerger::new(expr_expectation);
        let Ok(sema_if_branch) = self.build_sema_if_branch(syn_if_branch, &mut merger) else {
            todo!()
        };
        let Ok(sema_elif_branches) = syn_elif_branches
            .iter()
            .map(|syn_elif_branch| self.build_sema_elif_branch(syn_elif_branch, &mut merger))
            .collect::<SynExprResultRef<Vec<_>>>()
        else {
            todo!()
        };
        let sema_else_branch = if let Some(syn_else_branch) = syn_else_branch {
            let Ok(sema_else_branch) = self.build_sema_else_branch(syn_else_branch, &mut merger)
            else {
                todo!()
            };
            Some(sema_else_branch)
            // merger.visit_branch(self, syn_else_branch);
        } else {
            None
        };
        // exhaustive iff else branch exists;
        (
            Ok(SemaStmtData::IfElse {
                sema_if_branch,
                sema_elif_branches,
                sema_else_branch,
            }),
            merger
                .merge(syn_else_branch.is_some(), &self.term_menu)
                .ok_or(DerivedSemaExprTypeError::BranchTypeMerge.into()),
        )
    }
}
