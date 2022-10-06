use super::*;

impl HuskyDevtime {
    pub(crate) fn feature_expr_figure(
        &self,
        expr: &Arc<FeatureLazyExpr>,
        is_specific: bool,
    ) -> Result<FigureCanvasData, (SampleId, __VMError)> {
        self.feature_repr_figure(&expr.clone().into(), is_specific)
    }
}
