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
            Some(_) => todo!(),
            None => todo!(),
        }
    }

    fn infer_new_last_stmt(&mut self, stmt_idx: StmtIdx) -> Option<LocalTerm> {
        todo!()
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
                        Some(_) => todo!(),
                        None => {
                            match initial_value {
                                Ok(initial_value) => {
                                    let ty = self.infer_new_expr(
                                        *initial_value,
                                        // ad hoc
                                        Expectation::None,
                                    );
                                    todo!()
                                }
                                Err(_) => todo!(),
                            }
                        }
                    },
                    Err(_) => todo!(),
                };
                todo!()
            }
            Stmt::Return { ref result, .. } => todo!(),
            Stmt::Require { ref condition, .. } => todo!(),
            Stmt::Assert { ref condition, .. } => todo!(),
            Stmt::Break { .. } => todo!(),
            Stmt::Eval { expr } => todo!(),
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
