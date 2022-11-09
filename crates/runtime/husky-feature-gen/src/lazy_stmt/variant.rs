use std::convert::Infallible;

use husky_ast::RawReturnContext;

use super::*;

#[derive(PartialEq, Eq, Clone)]
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
        return_context: Infallible,
    },
    Return {
        result: Arc<FeatureLazyExpr>,
    },
    ReturnUnveil {
        return_context: Infallible,
        result: Arc<FeatureLazyExpr>,
        implicit_conversion: ImplicitConversion,
    },
    ReturnXml {
        result: Arc<FeatureXmlExpr>,
    },
    ConditionFlow {
        branches: Vec<Arc<FeatureLazyBranch>>,
    },
}

impl std::fmt::Debug for FeatureLazyStmtVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Init { varname, value } => f.debug_struct("Init").finish(),
            Self::Assert { condition } => f.debug_struct("Assert").finish(),
            Self::Require {
                condition,
                return_context,
            } => f.debug_struct("Require").finish(),
            Self::Return { result } => f.debug_struct("Return").finish(),
            Self::ReturnUnveil {
                return_context,
                result,
                implicit_conversion,
            } => f.debug_struct("ReturnUnveil").finish(),
            Self::ReturnXml { result } => f.debug_struct("ReturnXml").finish(),
            Self::ConditionFlow { branches } => f.debug_struct("ConditionFlow").finish(),
        }
    }
}

impl FeatureLazyStmtVariant {
    pub(super) fn opt_feature(&self, feature_interner: &FeatureInterner) -> Option<FeatureItd> {
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
            FeatureLazyStmtVariant::ReturnUnveil {
                result,
                implicit_conversion,
                ..
            } => Some(feature_interner.intern(Feature::ReturnUnveil {
                result: result.feature,
                implicit_conversion: *implicit_conversion,
            })),
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
