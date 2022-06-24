use crate::{eval_id::FeatureEvalId, *};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FeatureBranch {
    pub block: Arc<FeatureLazyBlock>,
    pub variant: FeatureBranchVariant,
    pub(crate) eval_id: FeatureEvalId,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FeatureBranchVariant {
    If { condition: Arc<FeatureLazyExpr> },
    Elif { condition: Arc<FeatureLazyExpr> },
    Else,
}
