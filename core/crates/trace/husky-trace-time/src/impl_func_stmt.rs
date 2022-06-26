use text::Text;
use vm::{History, VMControl};

use super::*;
use crate::*;

impl HuskyTraceTime {
    pub(crate) fn func_stmt_figure(&self, stmt: &FuncStmt, history: &History) -> FigureCanvasData {
        match stmt.variant {
            FuncStmtVariant::Init {
                varname,
                ref initial_value,
            } => todo!(),
            FuncStmtVariant::Assert { ref condition } => todo!(),
            FuncStmtVariant::Return { ref result } => todo!(),
            FuncStmtVariant::ConditionFlow { ref branches } => todo!(),
            FuncStmtVariant::Match {
                ref match_expr,
                ref branches,
            } => todo!(),
        }
    }
}
