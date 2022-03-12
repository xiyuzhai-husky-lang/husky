use crate::*;

use std::sync::Arc;

use feature::LazyBlock;

impl TraceFactory {
    pub fn lazy_block_subtraces(
        &self,
        parent: &Trace,
        lazy_block: &LazyBlock,
        text: &Text,
    ) -> Vec<Arc<Trace>> {
        lazy_block
            .stmts
            .iter()
            .map(|stmt| self.lazy_stmt_traces(parent, stmt.clone(), text))
            .flatten()
            .collect()
    }
}
