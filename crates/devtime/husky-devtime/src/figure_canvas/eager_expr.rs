use super::*;

impl HuskyDevtime {
    pub(crate) fn eager_expr_figure(
        &self,
        expr: &EagerExpr,
        history: &History<'static>,
    ) -> SpecificFigureCanvasData {
        if let Some(entry) = history.get(expr) {
            match entry {
                HistoryEntry::PureExpr { result, .. } => match result {
                    Ok(output) => SpecificFigureCanvasData::new(
                        self.visualize_temp_value(
                            output,
                            expr.intrinsic_ty(),
                            expr.file,
                            expr.range,
                        )
                        .unwrap(),
                    ),
                    Err(_) => SpecificFigureCanvasData::void(),
                },
                HistoryEntry::Exec { .. } => todo!(),
                HistoryEntry::Loop { .. } => panic!(),
                HistoryEntry::ControlFlow { .. } => todo!(),
                HistoryEntry::Break => todo!(),
                HistoryEntry::PatternMatching { .. } => todo!(),
            }
        } else {
            SpecificFigureCanvasData::void()
        }
    }
}
