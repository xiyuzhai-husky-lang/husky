use super::*;

impl<Task: IsTask> Devtime<Task> {
    pub(crate) fn feature_expr_specific_figure(
        &self,
        expr: &ValExpr,
    ) -> Result<SpecificFigureCanvasData, (SampleId, __VMError)> {
        self.feature_repr_specific_figure(&expr.clone().into())
    }

    pub(crate) fn feature_expr_generic_figure(
        &self,
        expr: &ValExpr,
    ) -> Result<GenericFigureCanvasData, (SampleId, __VMError)> {
        self.feature_repr_generic_figure(&expr.clone().into())
    }
}
