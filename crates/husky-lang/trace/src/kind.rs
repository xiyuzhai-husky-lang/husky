use feature::{FeatureBlock, FeatureBranch, FeatureExpr, FeatureStmt};
use semantics::DeclStmt;

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TraceKind {
    Main(Arc<FeatureBlock>),
    FeatureStmt(Arc<FeatureStmt>),
    FeatureBranch(Arc<FeatureBranch>),
    FeatureExpr(Arc<FeatureExpr>),
    Input(Arc<FeatureExpr>),
    DeclStmt {
        stmt: Arc<DeclStmt>,
        tokens: Vec<TokenProps>,
        control_signal: TraceInterpreterControlSignal,
    },
}

impl Serialize for TraceKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(match self {
            TraceKind::Main(_) => "Main",
            TraceKind::FeatureStmt(_) => "FeatureStmt",
            TraceKind::FeatureBranch(_) => "FeatureBranch",
            TraceKind::FeatureExpr(_) => "FeatureExpr",
            TraceKind::Input(_) => "Input",
            TraceKind::DeclStmt { .. } => "DeclStmt",
        })
    }
}
