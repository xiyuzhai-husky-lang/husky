use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FeatureStmt {
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
}
