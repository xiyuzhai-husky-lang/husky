use crate::*;

use std::sync::Arc;

impl<'eval> TraceFactory<'eval> {
    pub fn feature_block_subtraces(
        &self,
        parent: &Trace<'eval>,
        feature_block: &FeatureBlock,
        text: &Text,
    ) -> Vec<Arc<Trace<'eval>>> {
        feature_block
            .stmts
            .iter()
            .map(|stmt| self.feature_stmt_traces(parent, stmt.clone(), text))
            .flatten()
            .collect()
    }
}
