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
            FeatureRepr::LazyExpr(_) => todo!(),
            FeatureRepr::LazyBlock(feature_block) => {
                self.feature_lazy_block_subtraces(parent, feature_block, text)
            }
            FeatureRepr::FuncBlock(_) => todo!(),
            FeatureRepr::ProcBlock(_) => todo!(),
        }
    }
}
