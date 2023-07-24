mod impl_func_stmt;
mod impl_proc_stmt;

use husky_item_semantics::EntityDefn;

use super::*;

impl Debugtime {
    pub(crate) fn feature_stmt_traces(
        &mut self,
        parent: &Trace,
        stmt: Arc<FeatureLazyStmt>,
    ) -> Vec<TraceId> {
        match stmt.variant {
            FeatureLazyStmtVariant::Init { .. }
            | FeatureLazyStmtVariant::Assert { .. }
            | FeatureLazyStmtVariant::Require { .. }
            | FeatureLazyStmtVariant::Return { .. }
            | FeatureLazyStmtVariant::ReturnUnveil { .. } => {
                vec![self.new_trace(
                    Some(parent.id()),
                    stmt.indent,
                    TraceVariant::FeatureStmt(stmt),
                )]
            }
            FeatureLazyStmtVariant::ConditionFlow { ref branches, .. } => branches
                .iter()
                .map(|branch| self.feature_branch_trace(parent, stmt.indent, branch.clone()))
                .collect(),
            FeatureLazyStmtVariant::ReturnHtml { .. } => todo!(),
        }
    }

    pub(crate) fn feature_branch_trace(
        &mut self,
        parent: &Trace,
        indent: Indent,
        branch: Arc<FeatureLazyBranch>,
    ) -> TraceId {
        self.new_trace(
            Some(parent.id()),
            indent,
            TraceVariant::FeatureBranch(branch),
        )
    }

    pub(crate) fn new_eager_expr_trace(
        &mut self,
        expr: Arc<EagerExpr>,
        history: Arc<History<'static>>,
        opt_parent: Option<&Trace>,
        indent: Indent,
    ) -> TraceId {
        self.new_trace(
            opt_parent.map(|parent| parent.id()),
            indent,
            TraceVariant::EagerExpr { expr, history },
        )
    }

    pub(crate) fn new_call_head_trace(&mut self, parent: &Trace, item: Arc<EntityDefn>) -> TraceId {
        return self.new_trace(Some(parent.id()), 0, TraceVariant::CallHead { item });
    }
}
