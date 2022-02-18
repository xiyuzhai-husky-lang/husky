use feature::{FeatureBlock, FeatureBranch, FeatureExpr, FeatureStmt};

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum TraceKind {
    Main(#[serde(skip)] Arc<FeatureBlock>),
    FeatureStmt(#[serde(skip)] Arc<FeatureStmt>),
    FeatureBranch(#[serde(skip)] Arc<FeatureBranch>),
    FeatureExpr(#[serde(skip)] Arc<FeatureExpr>),
}
