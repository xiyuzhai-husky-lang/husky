use feature::{FeatureBlock, FeatureExpr, FeatureStmt};
use file::FilePtr;

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TraceKind {
    Mock {
        tokens: Vec<TraceToken>,
    },
    Main {
        main_file: FilePtr,
        feature_block: Arc<FeatureBlock>,
    },
    Stmt(Arc<FeatureStmt>),
    Expr(Arc<FeatureExpr>),
}
