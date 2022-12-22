use crate::*;

impl TraceVariant {
    pub fn file_and_range(&self) -> (DiffPath, TextRange) {
        match self {
            TraceVariant::Main(repr) => (repr.file(), repr.text_range()),
            TraceVariant::Module { file, range, .. } => (*file, *range),
            TraceVariant::EntityFeature { repr, .. } => (repr.file(), repr.text_range()),
            TraceVariant::FeatureStmt(stmt) => (stmt.file, stmt.range),
            TraceVariant::FeatureExpr(expr) => (expr.expr.file, expr.expr.range),
            TraceVariant::FeatureBranch(branch) => (branch.block.file, branch.block.range),
            TraceVariant::FeatureCallArgument { argument, .. } => {
                (argument.expr.file, argument.expr.range)
            }
            TraceVariant::FuncStmt { stmt, .. } => (stmt.file, stmt.range),
            TraceVariant::EagerExpr { expr, .. } => (expr.file, expr.range),
            TraceVariant::CallHead { entity, .. } => (entity.file, entity.range),
            TraceVariant::ProcStmt { stmt, .. } => (stmt.file, stmt.range),
            TraceVariant::LoopFrame { loop_stmt, .. } => (loop_stmt.file, loop_stmt.range),
            TraceVariant::ProcBranch { branch, .. } => (branch.file, branch.range),
            TraceVariant::FuncBranch { branch, .. } => (branch.file, branch.range),
            TraceVariant::EagerCallArgument { argument, .. } => (argument.file, argument.range),
        }
    }
}
