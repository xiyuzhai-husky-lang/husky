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
            FuncStmtVariant::Return { ref result } => {
                self.eager_expr_tokens(result, history, ExprTokenConfig::stmt())
            }
            FuncStmtVariant::Match {
                ref match_expr,
                ref branches,
            } => todo!(),
            FuncStmtVariant::ConditionFlow { .. } => panic!(),
        }
    }
}
