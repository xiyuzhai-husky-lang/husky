use crate::*;

use super::eval_feature_stmt_trace;

use std::sync::Arc;

use feature::FeatureBlock;

pub fn eval_feature_block_subtraces(
    parent: Option<usize>,
    feature_block: &FeatureBlock,
) -> Arc<Vec<Arc<Trace>>> {
    Arc::new(
        feature_block
            .stmts
            .iter()
            .map(|stmt| eval_feature_stmt_trace(parent, stmt.clone()))
            .collect(),
    )
}
