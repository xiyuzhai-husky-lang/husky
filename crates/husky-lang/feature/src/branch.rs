use bitvec::prelude::BitVec;

use crate::{eval::FeatureEvalId, *};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LazyBranch {
    pub block: LazyBlock,
    pub kind: LazyBranchKind,
    pub(crate) eval_id: FeatureEvalId,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LazyBranchKind {
    If { condition: Arc<LazyExpr> },
    Elif { condition: Arc<LazyExpr> },
    Else,
}
