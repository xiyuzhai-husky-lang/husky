use super::*;

impl HuskyTraceTime {
    pub(crate) fn feature_stmt_figure(
        &self,
        stmt: &FeatureStmt,
    ) -> Result<FigureCanvasData, (SampleId, EvalError)> {
        match stmt.variant {
            FeatureLazyStmtVariant::Init { varname, ref value } => self.feature_expr_figure(value),
            FeatureLazyStmtVariant::Assert { .. } => Ok(FigureCanvasData::void()),
            FeatureLazyStmtVariant::Return { ref result } => self.feature_expr_figure(result),
            FeatureLazyStmtVariant::ConditionFlow { ref branches } => todo!(),
            FeatureLazyStmtVariant::ReturnXml { ref result } => todo!(),
        }
    }
}
