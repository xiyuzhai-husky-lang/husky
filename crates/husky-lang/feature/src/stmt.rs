use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FeatureStmt {
    pub(crate) kind: FeatureStmtKind,
    pub(crate) feature: Option<FeaturePtr>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FeatureStmtKind {
    Init {
        varname: CustomIdentifier,
        value: FeatureExpr,
    },
    Assert {
        condition: FeatureExpr,
    },
    Return {
        result: FeatureExpr,
    },
}
