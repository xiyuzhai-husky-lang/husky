use husky_text::HuskyText;
use vm::{History, VMControl};

use super::*;
use crate::*;

impl HuskyTraceTime {
    pub(crate) fn func_stmt_figure(
        &self,
        stmt: &FuncStmt,
        history: &History<'static>,
    ) -> FigureCanvasData {
        match stmt.variant {
            FuncStmtVariant::Init {
                varname,
                ref initial_value,
            } => self.eager_expr_figure(initial_value, history),
            FuncStmtVariant::Assert { ref condition } => FigureCanvasData::void(),
            FuncStmtVariant::Return { ref result } => self.eager_expr_figure(result, history),
            FuncStmtVariant::ConditionFlow { ref branches } => todo!(),
            FuncStmtVariant::Match {
                ref match_expr,
                ref branches,
            } => todo!(),
        }
    }
}
