use feature::{FeatureBlock, FeatureBranch, FeatureExpr, FeatureStmt};
use semantics::{DeclStmt, Entity, Expr, ImprStmt};

use crate::{interpreter::TraceStackValue, *};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TraceKind {
    Main(Arc<FeatureBlock>),
    FeatureStmt(Arc<FeatureStmt>),
    FeatureBranch(Arc<FeatureBranch>),
    FeatureExpr(Arc<FeatureExpr>),
    Input(Arc<FeatureExpr>),
    StrictDeclStmt {
        stmt: Arc<DeclStmt>,
        tokens: Vec<TokenProps>,
        control_signal: TraceInterpreterControlSignal,
    },
    ImprStmt {
        stmt: Arc<ImprStmt>,
        tokens: Vec<TokenProps>,
        control_signal: TraceInterpreterControlSignal,
    },
    Expr {
        expr: Arc<Expr>,
        value: TraceStackValue,
    },
    CallHead {
        entity: Arc<Entity>,
        tokens: Vec<TokenProps>,
    },
}

impl TraceKind {
    pub fn file_and_range(&self) -> (FilePtr, TextRange) {
        match self {
            TraceKind::Main(ref block) => (block.file, block.range),
            TraceKind::FeatureStmt(ref stmt) => (stmt.file, stmt.range),
            TraceKind::FeatureExpr(ref expr) => (expr.file, expr.range),
            TraceKind::FeatureBranch(ref branch) => (branch.block.file, branch.block.range),
            TraceKind::Input(_) => todo!(),
            TraceKind::StrictDeclStmt { ref stmt, .. } => (stmt.file, stmt.range),
            TraceKind::Expr { ref expr, .. } => (expr.file, expr.range),
            TraceKind::CallHead { ref entity, .. } => (entity.file, entity.range),
            TraceKind::ImprStmt { stmt, .. } => (stmt.file, stmt.range),
        }
    }
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
            TraceKind::StrictDeclStmt { .. } => "StrictDeclStmt",
            TraceKind::ImprStmt { .. } => "ImprStmt",
            TraceKind::Expr { .. } => "Expr",
            TraceKind::CallHead { .. } => "CallHead",
        })
    }
}
