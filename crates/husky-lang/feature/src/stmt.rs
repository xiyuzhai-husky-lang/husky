use file::FilePtr;
use semantics::{DeclBranchGroupKind, DeclBranchKind};
use text::TextRange;

use crate::{eval::FeatureEvalId, *};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LazyStmt {
    pub indent: fold::Indent,
    pub kind: LazyStmtKind,
    pub(crate) feature: Option<FeaturePtr>,
    pub file: FilePtr,
    pub range: TextRange,
    pub eval_id: FeatureEvalId,
}

impl text::TextRanged for LazyStmt {
    fn text_range_ref(&self) -> &text::TextRange {
        &self.range
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LazyStmtKind {
    Init {
        varname: CustomIdentifier,
        value: Arc<LazyExpr>,
    },
    Assert {
        condition: Arc<LazyExpr>,
    },
    Return {
        result: Arc<LazyExpr>,
    },
    Branches {
        kind: DeclBranchGroupKind,
        branches: Vec<Arc<LazyBranch>>,
    },
}
