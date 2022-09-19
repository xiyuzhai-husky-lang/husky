use super::*;

impl Tracetime {
    pub(crate) fn feature_stmt_figure(
        &self,
        stmt: &FeatureLazyStmt,
        is_specific: bool,
    ) -> Result<FigureCanvasData, (SampleId, __VMError)> {
        match stmt.variant {
            FeatureLazyStmtVariant::Init { ref value, .. } => {
                self.feature_expr_figure(value, is_specific)
            }
            FeatureLazyStmtVariant::Assert { .. } => Ok(FigureCanvasData::void()),
            FeatureLazyStmtVariant::Return { ref result } => {
                self.feature_expr_figure(result, is_specific)
            }
            FeatureLazyStmtVariant::ReturnUnveil { ref result, .. } => {
                self.feature_expr_figure(result, is_specific)
            }
            FeatureLazyStmtVariant::ConditionFlow { .. } => todo!(),
            FeatureLazyStmtVariant::ReturnXml { .. } => todo!(),
            FeatureLazyStmtVariant::Require { .. } => Ok(FigureCanvasData::void()),
        }
    }
}
