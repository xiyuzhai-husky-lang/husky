use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn infer_new_block(
        &mut self,
        stmts: StmtIdxRange,
        expr_expectation: Expectation,
    ) -> Option<LocalTerm> {
        for stmt in stmts.start()..(stmts.end() - 1) {
            self.infer_new_nonlast_stmt(stmt)
        }
        self.infer_new_last_stmt(stmts.end() - 1, expr_expectation)
    }
    fn visit_new_exprs_in_stmt(&mut self, stmt_idx: StmtIdx) {}

    fn infer_new_nonlast_stmt(&mut self, stmt_idx: StmtIdx) {
        match self.calc_stmt(stmt_idx, Expectation::UnitOrNever) {
            Some(ty) => match ty {
                LocalTerm::Resolved(ty) => match ty {
                    ty if ty == self.term_menu.unit() => return,
                    ty if ty == self.term_menu.never() => return,
                    ty => todo!(),
                },
                LocalTerm::Unresolved(_) => todo!(),
            },
            None => (),
        }
    }

    fn infer_new_last_stmt(
        &mut self,
        stmt_idx: StmtIdx,
        expr_expectation: Expectation,
    ) -> Option<LocalTerm> {
        let expr_expectation = match self.return_ty {
            Some(_) => todo!(),
            None => Expectation::None,
        };
        self.calc_stmt(stmt_idx, expr_expectation)
    }

    fn calc_stmt(&mut self, stmt_idx: StmtIdx, expr_expectation: Expectation) -> Option<LocalTerm> {
        match self.expr_region_data[stmt_idx] {
            Stmt::Let {
                let_token,
                ref let_variable_pattern,
                ref initial_value,
                ..
            } => self.calc_let_stmt(let_variable_pattern, initial_value),
            Stmt::Return { ref result, .. } => {
                result.as_ref().copied().map(|result| {
                    self.infer_new_expr(result, Expectation::Return { ty: self.return_ty })
                });
                todo!()
            }
            Stmt::Require { ref condition, .. } => {
                condition
                    .as_ref()
                    .copied()
                    .map(|condition| self.infer_new_expr(condition, Expectation::Condition));
                Some(self.term_menu.unit().into())
            }
            Stmt::Assert { ref condition, .. } => todo!(),
            Stmt::Break { .. } => Some(self.term_menu.never().into()),
            Stmt::Eval { expr_idx } => self.infer_new_expr(expr_idx, expr_expectation),
            Stmt::ForBetween {
                ref particulars,
                ref block,
                ..
            } => todo!(),
            Stmt::ForIn {
                ref condition,
                ref block,
                ..
            } => todo!(),
            Stmt::ForExt { ref block, .. } => todo!(),
            Stmt::While {
                ref condition,
                ref block,
                ..
            } => {
                condition
                    .as_ref()
                    .copied()
                    .map(|condition| self.infer_new_expr(condition, Expectation::Condition));
                todo!()
            }
            Stmt::DoWhile {
                do_token,
                while_token,
                ref condition,
                ref eol_colon,
                ref block,
            } => {
                condition
                    .as_ref()
                    .copied()
                    .map(|condition| self.infer_new_expr(condition, Expectation::Condition));
                todo!()
            }
            Stmt::IfElse {
                ref if_branch,
                ref elif_branches,
                ref else_branch,
            } => self.calc_if_else_stmt(
                if_branch,
                elif_branches,
                else_branch.as_ref(),
                expr_expectation,
            ),
            Stmt::Match {} => todo!(),
            Stmt::Err(_) => todo!(),
        }
    }

    fn calc_let_stmt(
        &mut self,
        let_variable_pattern: &Result<LetVariablesPattern, ExprError>,
        initial_value: &Result<idx_arena::ArenaIdx<Expr>, ExprError>,
    ) -> Option<LocalTerm> {
        let pattern_ty = match let_variable_pattern {
            Ok(pattern) => match pattern.ty() {
                Some(ty) => todo!(),
                None => {
                    initial_value
                        .as_ref()
                        .ok()
                        .map(|initial_value| {
                            self.infer_new_expr(
                                *initial_value,
                                // ad hoc
                                Expectation::None,
                            )
                        })
                        .flatten()
                }
            },
            Err(_) => todo!(),
        };
        match pattern_ty {
            Some(ty) if ty == self.term_menu.never().into() => todo!(),
            Some(ty) => todo!(),
            None => (),
        }
        Some(self.term_menu.unit().into())
    }

    fn calc_if_else_stmt(
        &mut self,
        if_branch: &IfBranch,
        elif_branches: &[ElifBranch],
        else_branch: Option<&ElseBranch>,
        mut expr_expectation: Expectation,
    ) -> Option<LocalTerm> {
        let mut ever_ty = None;
        let mut has_error = false;
        if_branch
            .condition
            .as_ref()
            .copied()
            .map(|condition| self.infer_new_expr(condition, Expectation::Condition));
        match if_branch.block {
            Ok(stmts) => match self.infer_new_block(stmts, expr_expectation) {
                Some(_) => todo!(),
                None => has_error = true,
            },
            Err(_) => has_error = true,
        };
        for elif_branch in elif_branches {
            todo!()
        }
        if let Some(else_branch) = else_branch {
            todo!()
        }
        if has_error {
            return None;
        }
        ever_ty
    }
}
