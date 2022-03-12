use feature::{LazyBlock, LazyBranch, LazyExpr, LazyStmt};
use semantics::{DeclStmt, Entity, Expr, ImprStmt};
use vm::{History, InstructionSheet, LoopFrameSnapshot, StackValueSnapshot, VMControl};

use crate::*;

#[derive(Debug, Clone)]
pub enum TraceKind {
    Main(Arc<LazyBlock>),
    LazyStmt(Arc<LazyStmt>),
    LazyBranch(Arc<LazyBranch>),
    LazyExpr(Arc<LazyExpr>),
    Input(Arc<LazyExpr>),
    StrictDeclStmt {
        stmt: Arc<DeclStmt>,
        history: Arc<History>,
    },
    ImprStmt {
        stmt: Arc<ImprStmt>,
        history: Arc<History>,
    },
    LoopFrame {
        loop_stmt: Arc<ImprStmt>,
        body_instruction_sheet: Arc<InstructionSheet>,
        body_stmts: Arc<Vec<Arc<ImprStmt>>>,
        loop_frame_snapshot: LoopFrameSnapshot,
    },
    StrictExpr {
        expr: Arc<Expr>,
        history: Arc<History>,
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
            TraceKind::LazyStmt(ref stmt) => (stmt.file, stmt.range),
            TraceKind::LazyExpr(ref expr) => (expr.file, expr.range),
            TraceKind::LazyBranch(ref branch) => (branch.block.file, branch.block.range),
            TraceKind::Input(_) => todo!(),
            TraceKind::StrictDeclStmt { ref stmt, .. } => (stmt.file, stmt.range),
            TraceKind::StrictExpr { ref expr, .. } => (expr.file, expr.range),
            TraceKind::CallHead { ref entity, .. } => (entity.file, entity.range),
            TraceKind::ImprStmt { stmt, .. } => (stmt.file, stmt.range),
            TraceKind::LoopFrame { loop_stmt, .. } => (loop_stmt.file, loop_stmt.range),
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
            TraceKind::LazyStmt(_) => "LazyStmt",
            TraceKind::LazyBranch(_) => "LazyBranch",
            TraceKind::LazyExpr(_) => "LazyExpr",
            TraceKind::Input(_) => "Input",
            TraceKind::StrictDeclStmt { .. } => "StrictDeclStmt",
            TraceKind::ImprStmt { .. } => "ImprStmt",
            TraceKind::StrictExpr { .. } => "StrictExpr",
            TraceKind::CallHead { .. } => "CallHead",
            TraceKind::LoopFrame { .. } => "LoopFrame",
        })
    }
}
