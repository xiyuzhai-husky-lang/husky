use husky_vm::History;

use super::*;
use crate::*;

impl Debugtime {
    pub(crate) fn func_stmt_figure(
        &self,
        stmt: &FuncStmt,
        history: &History<'static>,
    ) -> SpecificFigureCanvasData {
        match stmt.variant {
            FuncStmtVariant::Init {
                ref initial_value, ..
            } => self.eager_expr_figure(initial_value, history).into(),
            FuncStmtVariant::Require { .. } | FuncStmtVariant::Assert { .. } => Default::default(),
            FuncStmtVariant::Return { ref result, .. } => {
                self.eager_expr_figure(result, history).into()
            }
            FuncStmtVariant::ConditionFlow { .. } => todo!(),
            FuncStmtVariant::Match { .. } => todo!(),
        }
    }
}
