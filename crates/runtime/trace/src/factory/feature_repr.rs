use crate::*;

use std::sync::Arc;

impl<'eval> TraceFactory<'eval> {
    pub fn feature_repr_subtraces(
        &self,
        parent: &Trace<'eval>,
        feature_repr: &FeatureRepr,
        text: &Text,
    ) -> Vec<Arc<Trace<'eval>>> {
        match feature_repr {
            FeatureRepr::Expr(_) => todo!(),
            FeatureRepr::LazyBlock(_) => todo!(),
            FeatureRepr::FuncBlock(_) => todo!(),
            FeatureRepr::ProcBlock(_) => todo!(),
        }
    }
}
