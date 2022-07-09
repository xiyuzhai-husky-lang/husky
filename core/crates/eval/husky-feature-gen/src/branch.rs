use crate::{eval_id::FeatureEvalId, *};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FeatureBranch {
    pub block: Arc<FeatureLazyBlock>,
    pub variant: FeatureBranchVariant,
    pub indicator: Arc<FeatureBranchIndicator>,
    pub(crate) eval_id: FeatureEvalId,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FeatureBranchVariant {
    If { condition: Arc<FeatureExpr> },
    Elif { condition: Arc<FeatureExpr> },
    Else,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FeatureBranchIndicator {
    variant: FeatureBranchIndicatorVariant,
}

impl FeatureBranchIndicator {
    pub fn new() -> Arc<Self> {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FeatureBranchIndicatorVariant {
    If { condition: Arc<FeatureExpr> },
    Elif { condition: Arc<FeatureExpr> },
    Else,
}
