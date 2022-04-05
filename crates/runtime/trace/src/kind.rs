use feature::*;
use vm::{History, InstructionSheet, LoopFrameSnapshot, StackValueSnapshot, VMControl};

use crate::*;

#[derive(Debug, Clone)]
pub enum TraceKind<'eval> {
    Main(Arc<FeatureBlock>),
    FeatureStmt(Arc<FeatureStmt>),
    FeatureBranch(Arc<FeatureBranch>),
    FeatureExpr(Arc<FeatureExpr>),
    Input(Arc<FeatureExpr>),
    StrictDeclStmt {
        stmt: Arc<FuncStmt>,
        history: Arc<History<'eval>>,
    },
    ImprStmt {
        stmt: Arc<ProcStmt>,
        history: Arc<History<'eval>>,
    },
    LoopFrame {
        loop_stmt: Arc<ProcStmt>,
        body_instruction_sheet: Arc<InstructionSheet>,
        body_stmts: Arc<Vec<Arc<ProcStmt>>>,
        loop_frame_snapshot: LoopFrameSnapshot<'eval>,
    },
    EagerExpr {
        expr: Arc<EagerExpr>,
        history: Arc<History<'eval>>,
    },
    CallHead {
        entity: Arc<Entity>,
        tokens: Vec<TokenProps<'eval>>,
    },
}

impl<'eval> TraceKind<'eval> {
    pub fn file_and_range(&self) -> (FilePtr, TextRange) {
        match self {
            TraceKind::Main(ref block) => (block.file, block.range),
            TraceKind::FeatureStmt(ref stmt) => (stmt.file, stmt.range),
            TraceKind::FeatureExpr(ref expr) => (expr.file, expr.range),
            TraceKind::FeatureBranch(ref branch) => (branch.block.file, branch.block.range),
            TraceKind::Input(_) => todo!(),
            TraceKind::StrictDeclStmt { ref stmt, .. } => (stmt.file, stmt.range),
            TraceKind::EagerExpr { ref expr, .. } => (expr.file, expr.range),
            TraceKind::CallHead { ref entity, .. } => (entity.file, entity.range),
            TraceKind::ImprStmt { stmt, .. } => (stmt.file, stmt.range),
            TraceKind::LoopFrame { loop_stmt, .. } => (loop_stmt.file, loop_stmt.range),
        }
    }
}

impl<'eval> Serialize for TraceKind<'eval> {
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
            TraceKind::EagerExpr { .. } => "StrictExpr",
            TraceKind::CallHead { .. } => "CallHead",
            TraceKind::LoopFrame { .. } => "LoopFrame",
        })
    }
}
