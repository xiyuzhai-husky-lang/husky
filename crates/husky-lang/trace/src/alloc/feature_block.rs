use crate::*;

use std::sync::Arc;

use feature::FeatureBlock;

impl TraceAllocator {
    pub fn feature_block_subtraces(
        &self,
        parent: &Trace,
        feature_block: &FeatureBlock,
    ) -> Vec<Arc<Trace>> {
        feature_block
            .stmts
            .iter()
            .map(|stmt| self.feature_stmt_traces(parent, stmt.clone()))
            .flatten()
            .collect()
    }
}
