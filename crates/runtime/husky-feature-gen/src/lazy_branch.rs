use crate::{eval_id::FeatureEvalId, *};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FeatureLazyBranch {
    pub block: Arc<FeatureLazyBlock>,
    pub variant: FeatureLazyBranchVariant,
    pub opt_arrival_indicator: Option<Arc<FeatureArrivalIndicator>>,
    pub(crate) eval_id: FeatureEvalId,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FeatureLazyBranchVariant {
    If { condition: Arc<FeatureLazyExpr> },
    Elif { condition: Arc<FeatureLazyExpr> },
    Else,
}

#[derive(PartialEq, Eq, Clone)]
pub struct FeatureArrivalIndicator {
    pub variant: FeatureArrivalIndicatorVariant,
    pub feature: FeaturePtr,
}

impl std::fmt::Debug for FeatureArrivalIndicator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FeatureArrivalIndicator")
            .field("variant", &self.variant)
            .finish()
    }
}

impl FeatureArrivalIndicator {
    pub fn new(
        variant: FeatureArrivalIndicatorVariant,
        feature_interner: &FeatureInterner,
    ) -> Arc<Self> {
        let feature = feature_interner.intern(match variant {
            FeatureArrivalIndicatorVariant::AfterStmtNotReturn { ref stmt } => {
                Feature::ArrivalAfterStmtNotReturn {
                    stmt: stmt.opt_feature.unwrap(),
                    opt_stmt_arrival_indicator: stmt
                        .opt_arrival_indicator
                        .as_ref()
                        .map(|ind| ind.feature),
                }
            }
            FeatureArrivalIndicatorVariant::AfterConditionNotMet {
                ref opt_parent,
                ref condition,
            } => Feature::ArrivalAfterConditionNotMet {
                opt_parent: opt_parent.as_ref().map(|p| p.feature),
                condition: condition.feature,
            },
            FeatureArrivalIndicatorVariant::IfConditionMet {
                ref opt_parent,
                ref condition,
            } => Feature::ArrivalIfConditionMet {
                opt_parent: opt_parent.as_ref().map(|p| p.feature),
                condition: condition.feature,
            },
        });
        Arc::new(Self { variant, feature })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FeatureArrivalIndicatorVariant {
    AfterStmtNotReturn {
        stmt: Arc<FeatureLazyStmt>,
    },
    AfterConditionNotMet {
        opt_parent: Option<Arc<FeatureArrivalIndicator>>,
        condition: Arc<FeatureLazyExpr>,
    },
    IfConditionMet {
        opt_parent: Option<Arc<FeatureArrivalIndicator>>,
        condition: Arc<FeatureLazyExpr>,
    },
}
