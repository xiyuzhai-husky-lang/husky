use husky_feature_eval::Division;
use husky_vm_primitive_value::PrimitiveValueData;

use super::*;

impl HuskyTraceTime {
    pub(crate) fn feature_expr_figure(
        &self,
        expr: &Arc<FeatureLazyExpr>,
    ) -> Result<FigureCanvasData, (SampleId, __VMError)> {
        self.feature_repr_figure(&expr.clone().into(), expr.opt_arrival_indicator.as_ref())
    }
}
