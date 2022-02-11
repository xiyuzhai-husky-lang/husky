use feature::{FeatureBlock, FeatureBranch, FeatureExpr, FeatureStmt};
use file::FilePtr;

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TraceKind {
    Main {
        main_file: FilePtr,
        feature_block: Arc<FeatureBlock>,
    },
    FeatureStmt(Arc<FeatureStmt>),
    FeatureExpr(Arc<FeatureExpr>),
    FeatureBranch(Arc<FeatureBranch>),
}
