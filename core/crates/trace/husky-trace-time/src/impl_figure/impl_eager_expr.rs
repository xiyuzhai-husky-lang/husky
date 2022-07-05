use check_utils::should_eq;

use super::*;

impl HuskyTraceTime {
    pub(crate) fn eager_expr_figure(
        &self,
        expr: &EagerExpr,
        history: &History<'static>,
    ) -> FigureCanvasData {
        if let Some(entry) = history.get(expr) {
            match entry {
                HistoryEntry::PureExpr { result } => match result {
                    Ok(output) => {
                        should_eq!(output.any_ref().ty_dyn(), expr.ty());
                        FigureCanvasData::new_specific(
                            self.visualize_temp_value(output, expr.ty(), expr.file, expr.range)
                                .unwrap(),
                        )
                    }
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
