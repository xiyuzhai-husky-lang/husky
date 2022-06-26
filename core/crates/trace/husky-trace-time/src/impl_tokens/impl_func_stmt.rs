use super::*;

impl HuskyTraceTime {
    pub(crate) fn func_stmt_tokens(
        &mut self,
        stmt: &FuncStmt,
        history: &Arc<History<'static>>,
    ) -> Vec<TraceTokenData> {
        match stmt.variant {
            FuncStmtVariant::Init {
                varname,
                ref initial_value,
            } => {
                let mut tokens = vec![];
                tokens.push(ident!(varname.ident.0));
                tokens.push(special!(" = "));
                tokens.extend(self.eager_expr_tokens(
                    initial_value,
                    history,
                    ExprTokenConfig::stmt(),
                ));
                tokens
            }
            FuncStmtVariant::Assert { ref condition } => {
                let mut tokens = vec![keyword!("assert ")];
                tokens.extend(self.eager_expr_tokens(condition, history, ExprTokenConfig::stmt()));
                tokens
            }
            FuncStmtVariant::Return { ref result } => {
                let mut tokens = vec![];
                tokens.extend(self.eager_expr_tokens(result, history, ExprTokenConfig::stmt()));
                tokens
            }
            FuncStmtVariant::Match {
                ref match_expr,
                ref branches,
            } => todo!(),
            FuncStmtVariant::ConditionFlow { .. } => panic!(),
        }
    }
}
