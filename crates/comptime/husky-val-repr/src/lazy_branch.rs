use crate::{eval_id::FeatureEvalId, *};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FeatureLazyBranch {
    pub block: ValBlock,
    pub variant: FeatureLazyBranchVariant,
    pub opt_arrival_indicator: Option<ValDomain>,
    pub(crate) eval_id: FeatureEvalId,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FeatureLazyBranchVariant {
    If { condition: ValExpr },
    Elif { condition: ValExpr },
    Else,
}

#[derive(PartialEq, Eq, Clone)]
pub struct ValDomain {
    pub variant: ValDomainData,
    pub feature: Val,
}

impl std::fmt::Debug for ValDomain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FeatureArrivalIndicator")
            .field("variant", &self.variant)
            .finish()
    }
}

impl ValDomain {
    pub fn new(variant: ValDomainData, feature_interner: &FeatureInterner) -> Arc<Self> {
        let feature = feature_interner.intern(match variant {
            ValDomainData::AfterStmtNotReturn { ref stmt } => Feature::ArrivalAfterStmtNotReturn {
                stmt: stmt.opt_feature.unwrap(),
                opt_stmt_arrival_indicator: stmt
                    .opt_arrival_indicator
                    .as_ref()
                    .map(|ind| ind.feature),
            },
            ValDomainData::AfterConditionNotMet {
                ref opt_parent,
                ref condition,
            } => Feature::ArrivalAfterConditionNotMet {
                opt_parent: opt_parent.as_ref().map(|p| p.feature),
                condition: condition.feature,
            },
            ValDomainData::IfConditionMet {
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
pub enum ValDomainData {
    AfterStmtNotReturn {
        stmt: Arc<ValStmt>,
    },
    AfterConditionNotMet {
        opt_parent: Option<ValDomain>,
        condition: ValExpr,
    },
    IfConditionMet {
        opt_parent: Option<ValDomain>,
        condition: ValExpr,
    },
}
