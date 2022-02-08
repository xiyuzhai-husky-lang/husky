use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FeatureStmt {
    pub indent: fold::Indent,
    pub kind: FeatureStmtKind,
    pub(crate) feature: Option<FeaturePtr>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FeatureStmtKind {
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
    Branch {
        conditional_feature_blocks: Vec<Arc<ConditionalFeatureBlock>>,
        default_feature_block: Option<Arc<FeatureBlock>>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ConditionalFeatureBlock {
    pub condition: Arc<FeatureExpr>,
    pub block: FeatureBlock,
}
