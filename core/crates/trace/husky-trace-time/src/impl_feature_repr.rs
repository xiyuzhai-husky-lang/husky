use crate::*;

use std::sync::Arc;

impl HuskyTraceTime {
    pub fn feature_repr_subtraces(
        &mut self,
        parent: &Trace,
        feature_repr: &FeatureRepr,
    ) -> Vec<TraceId> {
        match feature_repr {
            FeatureRepr::Value { .. } => todo!(),
            FeatureRepr::Expr(_) => todo!(),
            FeatureRepr::LazyBlock(feature_block) => {
                self.feature_lazy_block_subtraces(parent, feature_block)
            }
            FeatureRepr::FuncBlock(_) => todo!(),
            FeatureRepr::ProcBlock(_) => todo!(),
        }
    }
}
