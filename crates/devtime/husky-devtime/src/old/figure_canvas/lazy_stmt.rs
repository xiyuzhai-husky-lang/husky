use super::*;

impl<Task: IsTask> Devtime<Task> {
    pub(crate) fn feature_stmt_specific_figure(
        &self,
        stmt: &ValStmt,
    ) -> Result<SpecificFigureCanvasData, (SampleId, __VMError)> {
        todo!()
        // match stmt.variant {
        //     ValStmtData::Init { ref value, .. } => self.feature_expr_specific_figure(value),
        //     ValStmtData::Assert { .. } => Ok(Default::default()),
        //     ValStmtData::Return { ref result } => self.feature_expr_specific_figure(result),
        //     ValStmtData::ReturnUnveil { ref result, .. } => {
        //         self.feature_expr_specific_figure(result)
        //     }
        //     ValStmtData::ConditionFlow { .. } => todo!(),
        //     ValStmtData::ReturnHtml { .. } => todo!(),
        //     ValStmtData::Require { .. } => Ok(Default::default()),
        // }
    }

    pub(crate) fn feature_stmt_generic_figure(
        &self,
        stmt: &ValStmt,
    ) -> Result<GenericFigureCanvasData, (SampleId, __VMError)> {
        todo!()
        // match stmt.variant {
        //     ValStmtData::Init { ref value, .. } => self.feature_expr_generic_figure(value),
        //     ValStmtData::Assert { .. } => Ok(Default::default()),
        //     ValStmtData::Return { ref result } => self.feature_expr_generic_figure(result),
        //     ValStmtData::ReturnUnveil { ref result, .. } => {
        //         self.feature_expr_generic_figure(result)
        //     }
        //     ValStmtData::ConditionFlow { .. } => todo!(),
        //     ValStmtData::ReturnHtml { .. } => todo!(),
        //     ValStmtData::Require { .. } => Ok(Default::default()),
        // }
    }
}
