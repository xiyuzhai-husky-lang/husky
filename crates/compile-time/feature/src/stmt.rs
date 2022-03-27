use file::FilePtr;
use semantics_lazy::LazyBranchGroupKind;
use text::TextRange;

use crate::{eval::FeatureEvalId, *};

#[derive(Debug, Clone)]
pub struct FeatureStmt {
    pub indent: fold::Indent,
    pub kind: FeatureStmtKind,
    pub(crate) feature: Option<FeaturePtr>,
    pub file: FilePtr,
    pub range: TextRange,
    pub eval_id: FeatureEvalId,
}

impl std::hash::Hash for FeatureStmt {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.eval_id.hash(state)
    }
}

impl PartialEq for FeatureStmt {
    fn eq(&self, other: &Self) -> bool {
        self.eval_id == other.eval_id
    }
}

impl Eq for FeatureStmt {}

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
    BranchGroup {
        kind: LazyBranchGroupKind,
        branches: Vec<Arc<FeatureBranch>>,
    },
}
