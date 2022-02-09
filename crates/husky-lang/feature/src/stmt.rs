use semantics::{DeclBranchKind, DeclBranchesKind};

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
    Branches {
        kind: DeclBranchesKind,
        branches: Vec<Arc<FeatureBranch>>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FeatureBranch {
    pub kind: FeatureBranchKind,
    pub block: FeatureBlock,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FeatureBranchKind {
    If { condition: Arc<FeatureExpr> },
    Elif { condition: Arc<FeatureExpr> },
    Else,
}
