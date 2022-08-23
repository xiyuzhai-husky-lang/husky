use husky_check_utils::should_eq;
use husky_vm_register_method::VMRegisterMethodX;

use super::*;

impl HuskyTraceTime {
    pub(crate) fn eager_expr_figure(
        &self,
        expr: &EagerExpr,
        history: &History<'static>,
    ) -> FigureCanvasData {
        if let Some(entry) = history.get(expr) {
            match entry {
                HistoryEntry::PureExpr { result, ty } => match result {
                    Ok(output) => FigureCanvasData::new_specific(
                        self.visualize_temp_value(output, expr.ty(), expr.file, expr.range)
                            .unwrap(),
                    ),
                    Err(e) => FigureCanvasData::void(),
                },
                HistoryEntry::Exec { .. } => todo!(),
                HistoryEntry::Loop { .. } => panic!(),
                HistoryEntry::ControlFlow {
                    opt_branch_entered: enter,
                    ..
                } => todo!(),
                HistoryEntry::Break => todo!(),
                HistoryEntry::PatternMatching { .. } => todo!(),
            }
        } else {
            FigureCanvasData::void()
        }
    }
}
