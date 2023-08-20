use crate::*;

impl TraceVariant {
    pub fn file_and_range(&self) -> (DiffPath, TextRange) {
        todo!()
        // match self {
        //     TraceVariant::Main(repr) => (repr.file(), repr.text_range()),
        //     TraceVariant::Module { file, range, .. } => (*file, *range),
        //     TraceVariant::EntityVal { repr, .. } => (repr.file(), repr.text_range()),
        //     TraceVariant::ValStmt(stmt) => (stmt.file, stmt.range),
        //     TraceVariant::ValExpr(expr) => (expr.expr.file, expr.expr.range),
        //     TraceVariant::ValBranch(branch) => (branch.block.file, branch.block.range),
        //     TraceVariant::ValCallArgument { argument, .. } => {
        //         (argument.expr.file, argument.expr.range)
        //     }
        //     TraceVariant::EagerExpr { expr, .. } => (expr.file, expr.range),
        //     TraceVariant::CallHead { item, .. } => (item.file, item.range),
        //     TraceVariant::EagerStmt { stmt, .. } => (stmt.file, stmt.range),
        //     TraceVariant::LoopFrame { loop_stmt, .. } => (loop_stmt.file, loop_stmt.range),
        //     TraceVariant::EagerBranch { branch, .. } => (branch.file, branch.range),
        //     TraceVariant::FuncBranch { branch, .. } => (branch.file, branch.range),
        //     TraceVariant::EagerCallArgument { argument, .. } => (argument.file, argument.range),
        // }
    }
}
