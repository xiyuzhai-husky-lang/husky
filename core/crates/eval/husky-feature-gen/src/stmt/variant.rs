use super::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FeatureLazyStmtVariant {
    Init {
        varname: CustomIdentifier,
        value: Arc<FeatureExpr>,
    },
    Assert {
        condition: Arc<FeatureExpr>,
    },
    Return {
        result: Arc<FeatureExpr>,
    },
    ReturnXml {
        result: Arc<FeatureXmlExpr>,
    },
    ConditionFlow {
        branches: Vec<Arc<FeatureBranch>>,
    },
}

impl FeatureLazyStmtVariant {
    pub(super) fn opt_feature(&self, feature_interner: &FeatureInterner) -> Option<FeaturePtr> {
        match self {
            FeatureLazyStmtVariant::Init { .. } => None,
            FeatureLazyStmtVariant::Assert { condition } => {
                Some(feature_interner.intern(Feature::Assert {
                    condition: condition.feature,
                }))
            }
            FeatureLazyStmtVariant::Return { result } => Some(result.feature),
            FeatureLazyStmtVariant::ReturnXml { result } => Some(result.feature),
            FeatureLazyStmtVariant::ConditionFlow { branches } => Some(
                feature_interner.intern(Feature::Branches {
                    branches: branches
                        .iter()
                        .map(|branch| match branch.variant {
                            FeatureBranchVariant::If { ref condition } => BranchedFeature {
                                condition: Some(condition.feature),
                                block: branch.block.feature,
                            },
                            FeatureBranchVariant::Elif { ref condition } => BranchedFeature {
                                condition: Some(condition.feature),
                                block: branch.block.feature,
                            },
                            FeatureBranchVariant::Else => BranchedFeature {
                                condition: None,
                                block: branch.block.feature,
                            },
                        })
                        .collect(),
                }),
            ),
        }
    }
}
