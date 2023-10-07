mod impl_func_stmt;
mod impl_proc_stmt;

use husky_syn_decl::SynDecl;

use super::*;

impl<Task: IsTask> Devtime<Task> {
    pub(crate) fn feature_stmt_traces(&mut self, parent: &Trace, stmt: ValStmt) -> Vec<TraceId> {
        todo!()
        // match stmt.variant {
        //     ValStmtData::Init { .. }
        //     | ValStmtData::Assert { .. }
        //     | ValStmtData::Require { .. }
        //     | ValStmtData::Return { .. }
        //     | ValStmtData::ReturnUnveil { .. } => {
        //         vec![self.new_trace(Some(parent.id()), stmt.indent, TraceVariant::ValStmt(stmt))]
        //     }
        //     ValStmtData::ConditionFlow { ref branches, .. } => branches
        //         .iter()
        //         .map(|branch| self.feature_branch_trace(parent, stmt.indent, branch.clone()))
        //         .collect(),
        //     ValStmtData::ReturnHtml { .. } => todo!(),
        // }
    }

    pub(crate) fn feature_branch_trace(
        &mut self,
        parent: &Trace,
        indent: Indent,
        branch: Arc<ValBranch>,
    ) -> TraceId {
        todo!()
        // self.new_trace(Some(parent.id()), indent, TraceVariant::ValBranch(branch))
    }

    pub(crate) fn new_eager_expr_trace(
        &mut self,
        expr: SynExprIdx,
        history: Arc<History>,
        opt_parent: Option<&Trace>,
        indent: Indent,
    ) -> TraceId {
        self.new_trace(
            opt_parent.map(|parent| parent.id()),
            indent,
            TraceVariant::EagerExpr { expr, history },
        )
    }

    pub(crate) fn new_call_head_trace(&mut self, parent: &Trace, item: SynDecl) -> TraceId {
        return self.new_trace(Some(parent.id()), 0, TraceVariant::CallHead { item });
    }
}
