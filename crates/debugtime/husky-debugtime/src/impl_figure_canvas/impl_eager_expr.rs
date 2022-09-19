use super::*;

impl Tracetime {
    pub(crate) fn eager_expr_figure(
        &self,
        expr: &EagerExpr,
        history: &History<'static>,
    ) -> FigureCanvasData {
        if let Some(entry) = history.get(expr) {
            match entry {
                HistoryEntry::PureExpr { result, .. } => match result {
                    Ok(output) => FigureCanvasData::new_specific(
                        self.visualize_temp_value(
                            output,
                            expr.intrinsic_ty(),
                            expr.file,
                            expr.range,
                        )
                        .unwrap(),
                    ),
                    Err(_) => FigureCanvasData::void(),
                },
                HistoryEntry::Exec { .. } => todo!(),
                HistoryEntry::Loop { .. } => panic!(),
                HistoryEntry::ControlFlow { .. } => todo!(),
                HistoryEntry::Break => todo!(),
                HistoryEntry::PatternMatching { .. } => todo!(),
            }
        } else {
            FigureCanvasData::void()
        }
    }
}
