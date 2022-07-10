use crate::{eval_id::FeatureEvalId, *};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FeatureBranch {
    pub block: Arc<FeatureLazyBlock>,
    pub variant: FeatureBranchVariant,
    pub indicator: Arc<FeatureArrivalIndicator>,
    pub(crate) eval_id: FeatureEvalId,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FeatureBranchVariant {
    If { condition: Arc<FeatureExpr> },
    Elif { condition: Arc<FeatureExpr> },
    Else,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FeatureArrivalIndicator {
    parent: Option<Arc<FeatureArrivalIndicator>>,
    variant: FeatureBranchIndicatorVariant,
    feature: FeaturePtr,
}

impl FeatureArrivalIndicator {
    pub fn new(
        variant: FeatureBranchIndicatorVariant,
        feature_interner: &FeatureInterner,
    ) -> Arc<Self> {
        let feature = match variant {
            FeatureBranchIndicatorVariant::AfterStmt { stmt } => Feature::ArrivalAfterStmt {
                stmt: stmt.opt_feature.unwrap(),
            },
            FeatureBranchIndicatorVariant::AfterCondition {
                opt_parent,
                condition,
            } => todo!(),
        };
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FeatureBranchIndicatorVariant {
    AfterStmt {
        stmt: Arc<FeatureStmt>,
    },
    AfterCondition {
        opt_parent: Option<Arc<FeatureArrivalIndicator>>,
        condition: Arc<FeatureExpr>,
    },
}
