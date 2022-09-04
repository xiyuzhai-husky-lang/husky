use husky_ast::RawReturnContext;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FeatureLazyStmtVariant {
    Init {
        varname: CustomIdentifier,
        value: Arc<FeatureLazyExpr>,
    },
    Assert {
        condition: Arc<FeatureLazyExpr>,
    },
    Require {
        condition: Arc<FeatureLazyExpr>,
        return_context: RawReturnContext,
    },
    Return {
        result: Arc<FeatureLazyExpr>,
    },
    ReturnUnveil {
        result: Arc<FeatureLazyExpr>,
    },
    ReturnXml {
        result: Arc<FeatureXmlExpr>,
    },
    ConditionFlow {
        branches: Vec<Arc<FeatureLazyBranch>>,
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
            FeatureLazyStmtVariant::Require { condition, .. } => {
                Some(feature_interner.intern(Feature::Require {
                    condition: condition.feature,
                }))
            }
            FeatureLazyStmtVariant::Return { result } => Some(result.feature),
            FeatureLazyStmtVariant::ReturnUnveil { result } => {
                Some(feature_interner.intern(Feature::ReturnUnveil {
                    result: result.feature,
                }))
            }
            FeatureLazyStmtVariant::ReturnXml { result } => Some(result.feature),
            FeatureLazyStmtVariant::ConditionFlow { branches } => Some(
                feature_interner.intern(Feature::Branches {
                    branches: branches
                        .iter()
                        .map(|branch| match branch.variant {
                            FeatureLazyBranchVariant::If { ref condition } => BranchedFeature {
                                condition: Some(condition.feature),
                                block: branch.block.feature,
                            },
                            FeatureLazyBranchVariant::Elif { ref condition } => BranchedFeature {
                                condition: Some(condition.feature),
                                block: branch.block.feature,
                            },
                            FeatureLazyBranchVariant::Else => BranchedFeature {
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
