use file::FilePtr;
use semantics::{DeclBranchKind, DeclBranchesKind};
use text::TextRange;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FeatureStmt {
    pub indent: fold::Indent,
    pub kind: FeatureStmtKind,
    pub(crate) feature: Option<FeaturePtr>,
    pub file: FilePtr,
    pub range: TextRange,
}

impl text::TextRanged for FeatureStmt {
    fn text_range_ref(&self) -> &text::TextRange {
        &self.range
    }
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
    pub block: FeatureBlock,
    pub kind: FeatureBranchKind,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FeatureBranchKind {
    If { condition: Arc<FeatureExpr> },
    Elif { condition: Arc<FeatureExpr> },
    Else,
}
