use crate::*;

use super::eval_stmt_trace;

use std::sync::Arc;

use feature::FeatureBlock;

pub fn eval_block_traces(parent: Option<usize>, feature_block: &FeatureBlock) -> Vec<Arc<Trace>> {
    feature_block
        .stmts
        .iter()
        .map(|stmt| eval_stmt_trace(parent, stmt.clone()))
        .collect()
}
