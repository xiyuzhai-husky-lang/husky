use crate::*;

use std::sync::Arc;

impl TraceFactory {
    pub fn feature_lazy_block_subtraces(
        &self,
        parent: &Trace,
        feature_block: &FeatureLazyBlock,
        text: &Text,
    ) -> Vec<Arc<Trace>> {
        feature_block
            .stmts
            .iter()
            .map(|stmt| self.feature_stmt_traces(parent, stmt.clone(), text))
            .flatten()
            .collect()
    }
}
