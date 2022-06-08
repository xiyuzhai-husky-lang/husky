use text::Text;
use vm::{History, VMControl};

use super::*;
use crate::*;

impl<'eval> TraceFactory<'eval> {
    pub fn new_func_stmt_trace(
        &self,
        parent_id: TraceId,
        indent: Indent,
        stmt: Arc<FuncStmt>,
        history: Arc<History<'eval>>,
        text: &Text,
    ) -> Arc<Trace<'eval>> {
        self.new_trace(
            Some(parent_id),
            indent,
            TraceVariant::FuncStmt { stmt, history },
            text,
        )
    }
    pub fn func_stmts_traces<'a>(
        &'a self,
        parent_id: TraceId,
        indent: Indent,
        stmts: &'a [Arc<FuncStmt>],
        text: &'a Text,
        history: &'a Arc<History<'eval>>,
    ) -> impl Iterator<Item = Arc<Trace<'eval>>> + 'a {
        stmts.iter().map(move |stmt| {
            self.new_func_stmt_trace(parent_id, indent, stmt.clone(), history.clone(), text)
        })
    }

    pub(super) fn func_stmt_lines(
        &self,
        stmt: &FuncStmt,
        text: &Text,
        history: &Arc<History<'eval>>,
    ) -> Vec<LineProps> {
        vec![LineProps {
            indent: stmt.indent,
            tokens: self.func_stmt_tokens(stmt, text, history),
            idx: 0,
        }]
    }

    pub(super) fn func_stmt_tokens(
        &self,
        stmt: &FuncStmt,
        text: &Text,
        history: &Arc<History<'eval>>,
    ) -> Vec<TraceTokenProps> {
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
                    text,
                    history,
                    ExprTokenConfig::stmt(),
                ));
                tokens
            }
            FuncStmtVariant::Assert { ref condition } => {
                let mut tokens = vec![keyword!("assert ")];
                tokens.extend(self.eager_expr_tokens(
                    condition,
                    text,
                    history,
                    ExprTokenConfig::stmt(),
                ));
                tokens
            }
            FuncStmtVariant::Return { ref result } => {
                let mut tokens = vec![];
                tokens.extend(self.eager_expr_tokens(
                    result,
                    text,
                    history,
                    ExprTokenConfig::stmt(),
                ));
                tokens
            }
            FuncStmtVariant::Match {
                ref match_expr,
                ref branches,
            } => todo!(),
            FuncStmtVariant::ConditionFlow { .. } => panic!(),
            FuncStmtVariant::ReturnXml { .. } => panic!(),
        }
    }
}
