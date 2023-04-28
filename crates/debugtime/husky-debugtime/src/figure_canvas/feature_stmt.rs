use super::*;

impl Debugtime {
    pub(crate) fn feature_stmt_specific_figure(
        &self,
        stmt: &FeatureLazyStmt,
    ) -> Result<SpecificFigureCanvasData, (SampleId, __VMError)> {
        match stmt.variant {
            FeatureLazyStmtVariant::Init { ref value, .. } => {
                self.feature_expr_specific_figure(value)
            }
            FeatureLazyStmtVariant::Assert { .. } => Ok(Default::default()),
            FeatureLazyStmtVariant::Return { ref result } => {
                self.feature_expr_specific_figure(result)
            }
            FeatureLazyStmtVariant::ReturnUnveil { ref result, .. } => {
                self.feature_expr_specific_figure(result)
            }
            FeatureLazyStmtVariant::ConditionFlow { .. } => todo!(),
            FeatureLazyStmtVariant::ReturnHtml { .. } => todo!(),
            FeatureLazyStmtVariant::Require { .. } => Ok(Default::default()),
        }
    }

    pub(crate) fn feature_stmt_generic_figure(
        &self,
        stmt: &FeatureLazyStmt,
    ) -> Result<GenericFigureCanvasData, (SampleId, __VMError)> {
        match stmt.variant {
            FeatureLazyStmtVariant::Init { ref value, .. } => {
                self.feature_expr_generic_figure(value)
            }
            FeatureLazyStmtVariant::Assert { .. } => Ok(Default::default()),
            FeatureLazyStmtVariant::Return { ref result } => {
                self.feature_expr_generic_figure(result)
            }
            FeatureLazyStmtVariant::ReturnUnveil { ref result, .. } => {
                self.feature_expr_generic_figure(result)
            }
            FeatureLazyStmtVariant::ConditionFlow { .. } => todo!(),
            FeatureLazyStmtVariant::ReturnHtml { .. } => todo!(),
            FeatureLazyStmtVariant::Require { .. } => Ok(Default::default()),
        }
    }
}
