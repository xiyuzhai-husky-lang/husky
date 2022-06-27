use crate::{eval_id::FeatureEvalId, *};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FeatureLazyBranch {
    pub block: Arc<FeatureLazyBlock>,
    pub variant: FeatureBranchVariant,
    pub(crate) eval_id: FeatureEvalId,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FeatureBranchVariant {
    If { condition: Arc<FeatureExpr> },
    Elif { condition: Arc<FeatureExpr> },
    Else,
}
