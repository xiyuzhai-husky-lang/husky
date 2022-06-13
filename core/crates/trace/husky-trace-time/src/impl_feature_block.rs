use crate::*;

use std::sync::Arc;

impl HuskyTraceTime {
    pub fn feature_lazy_block_subtraces(
        &mut self,
        parent: &Trace,
        feature_block: &FeatureLazyBlock,
    ) -> Vec<TraceId> {
        feature_block
            .stmts
            .iter()
            .map(|stmt| self.feature_stmt_traces(parent, stmt.clone()))
            .flatten()
            .collect()
    }
}
