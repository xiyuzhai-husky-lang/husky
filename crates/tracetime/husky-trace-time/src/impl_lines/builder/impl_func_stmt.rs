use super::*;

impl<'a> TraceTokenBuilder<'a> {
    pub(crate) fn func_stmt_tokens(&mut self, stmt: &FuncStmt, history: &Arc<History<'static>>) {
        match stmt.variant {
            FuncStmtVariant::Init {
                varname,
                ref initial_value,
            } => {
                self.push(ident!(varname.ident.0));
                self.push(special!(" = "));
                self.eager_expr_tokens(initial_value, history, ExprTokenConfig::stmt())
            }
            FuncStmtVariant::Assert { ref condition } => {
                self.push(keyword!("assert "));
                self.eager_expr_tokens(condition, history, ExprTokenConfig::stmt())
            }
            FuncStmtVariant::Require { ref condition, .. } => {
                self.push(keyword!("require "));
                self.eager_expr_tokens(condition, history, ExprTokenConfig::stmt())
            }
            FuncStmtVariant::Return { ref result, .. } => {
                self.eager_expr_tokens(result, history, ExprTokenConfig::stmt())
            }
            FuncStmtVariant::Match {
                ref match_expr,
                ref branches,
            } => todo!(),
            FuncStmtVariant::ConditionFlow { .. } => panic!(),
        }
    }

    pub(crate) fn func_branch_tokens(
        &mut self,
        stmt: &FuncStmt,
        branch: &FuncConditionFlowBranch,
        history: &Arc<History<'static>>,
    ) {
        match branch.variant {
            FuncConditionFlowBranchVariant::If { ref condition } => {
                self.push(keyword!("if "));
                self.eager_expr_tokens(condition, history, ExprTokenConfig::branch());
                self.push(special!(":"))
            }
            FuncConditionFlowBranchVariant::Elif { ref condition } => {
                self.push(keyword!("elif "));
                self.eager_expr_tokens(condition, history, ExprTokenConfig::branch());
                self.push(special!(":"))
            }
            FuncConditionFlowBranchVariant::Else => {
                self.push(keyword!("else"));
                self.push(special!(":"))
            }
        }
        if let Some(entry) = history.get(stmt) {
            match entry {
                HistoryEntry::ControlFlow { control, .. } => self.add_control_tokens(control),
                _ => todo!(),
            }
        }
    }
}
