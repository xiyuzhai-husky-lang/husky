use crate::*;

impl<'eval> Trace<'eval> {
    pub fn figure(&self) -> Option<FigureProps> {
        match self.kind {
            TraceKind::Main(_) => None,
            TraceKind::FeatureStmt(ref stmt) => match stmt.kind {
                FeatureStmtKind::Init { varname, ref value } => todo!(),
                FeatureStmtKind::Assert { .. } => todo!(),
                FeatureStmtKind::Return { ref result } => todo!(),
                FeatureStmtKind::BranchGroup { kind, ref branches } => todo!(),
            },
            TraceKind::FeatureBranch(_) => todo!(),
            TraceKind::FeatureExpr(_) => todo!(),
            TraceKind::Input(_) => todo!(),
            TraceKind::StrictDeclStmt {
                ref stmt,
                ref history,
            } => todo!(),
            TraceKind::ImprStmt {
                ref stmt,
                ref history,
            } => match stmt.kind {
                ImprStmtKind::Init {
                    varname,
                    ref initial_value,
                    init_kind,
                    varidx,
                } => todo!(),
                ImprStmtKind::Assert { .. } => todo!(),
                ImprStmtKind::Execute { .. } => todo!(),
                ImprStmtKind::Return { ref result } => todo!(),
                ImprStmtKind::BranchGroup { kind, ref branches } => todo!(),
                ImprStmtKind::Loop {
                    ref loop_kind,
                    ref stmts,
                } => todo!(),
            },
            TraceKind::LoopFrame { .. } => todo!(),
            TraceKind::EagerExpr {
                ref expr,
                ref history,
            } => todo!(),
            TraceKind::CallHead {
                ref entity,
                ref tokens,
            } => todo!(),
        }
    }
}
