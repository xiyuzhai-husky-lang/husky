use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn infer_new_stmts(&mut self, stmts: StmtIdxRange) -> Option<LocalTerm> {
        for stmt in stmts.start()..(stmts.end() - 1) {
            self.infer_new_nonlast_stmt(stmt)
        }
        self.infer_new_last_stmt(stmts.end() - 1)
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

    fn infer_new_last_stmt(&mut self, stmt_idx: StmtIdx) -> Option<LocalTerm> {
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
            } => {
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
                    Some(_) => todo!(),
                    None => (),
                }
                Some(self.term_menu.unit().into())
            }
            Stmt::Return { ref result, .. } => todo!(),
            Stmt::Require { ref condition, .. } => todo!(),
            Stmt::Assert { ref condition, .. } => todo!(),
            Stmt::Break { .. } => todo!(),
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
                while_token,
                ref condition,
                ref eol_colon,
                ref block,
            } => todo!(),
            Stmt::DoWhile {
                do_token,
                while_token,
                ref condition,
                ref eol_colon,
                ref block,
            } => todo!(),
            Stmt::IfElse {
                ref if_branch,
                ref elif_branches,
                ref else_branch,
            } => todo!(),
            Stmt::Match {} => todo!(),
            Stmt::Err(_) => todo!(),
        }
    }
}
