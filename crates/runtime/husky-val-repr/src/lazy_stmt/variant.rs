use std::convert::Infallible;

use super::*;

#[derive(PartialEq, Eq, Clone)]
pub enum FeatureLazyStmtVariant {
    Init {
        varname: Ident,
        value: ValExpr,
    },
    Assert {
        condition: ValExpr,
    },
    Require {
        condition: ValExpr,
        return_context: Infallible,
    },
    Return {
        result: ValExpr,
    },
    ReturnUnveil {
        return_context: Infallible,
        result: ValExpr,
        implicit_conversion: ImplicitConversion,
    },
    ReturnHtml {
        result: Arc<FeatureHtmlExpr>,
    },
    ConditionFlow {
        branches: Vec<Arc<FeatureLazyBranch>>,
    },
}

impl std::fmt::Debug for FeatureLazyStmtVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Init {
                varname: _,
                value: _,
            } => f.debug_struct("Init").finish(),
            Self::Assert { condition: _ } => f.debug_struct("Assert").finish(),
            Self::Require {
                condition: _,
                return_context: _,
            } => f.debug_struct("Require").finish(),
            Self::Return { result: _ } => f.debug_struct("Return").finish(),
            Self::ReturnUnveil {
                return_context: _,
                result: _,
                implicit_conversion: _,
            } => f.debug_struct("ReturnUnveil").finish(),
            Self::ReturnHtml { result: _ } => f.debug_struct("ReturnHtml").finish(),
            Self::ConditionFlow { branches: _ } => f.debug_struct("ConditionFlow").finish(),
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
            FeatureLazyStmtVariant::ReturnHtml { result } => Some(result.feature),
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
