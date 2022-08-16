use super::*;

impl HuskyTraceTime {
    pub(crate) fn feature_stmt_figure(
        &self,
        stmt: &FeatureStmt,
    ) -> Result<FigureCanvasData, (SampleId, __VMError)> {
        match stmt.variant {
            FeatureStmtVariant::Init { varname, ref value } => self.feature_expr_figure(value),
            FeatureStmtVariant::Assert { .. } => Ok(FigureCanvasData::void()),
            FeatureStmtVariant::Return { ref result } => self.feature_expr_figure(result),
            FeatureStmtVariant::ConditionFlow { ref branches } => todo!(),
            FeatureStmtVariant::ReturnXml { ref result } => todo!(),
            FeatureStmtVariant::Require { ref condition, .. } => Ok(FigureCanvasData::void()),
        }
    }
}
