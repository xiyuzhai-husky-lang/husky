use feature::{LazyBranch, LazyBranchKind};

use crate::*;

use super::expr::ExprTokenConfig;

impl TraceFactory {
    pub fn lazy_branch_trace(
        &self,
        parent: &Trace,
        indent: Indent,
        branch: Arc<LazyBranch>,
        text: &Text,
    ) -> Arc<Trace> {
        self.new_trace(Some(parent.id), indent, TraceKind::LazyBranch(branch), text)
    }

    pub(super) fn lazy_branch_tokens(&self, branch: &LazyBranch, text: &Text) -> Vec<TokenProps> {
        match branch.kind {
            LazyBranchKind::If { ref condition } => {
                let mut tokens = vec![keyword!("if ")];
                tokens.extend(self.lazy_expr_tokens(condition, text, ExprTokenConfig::branch()));
                tokens
            }
            LazyBranchKind::Elif { ref condition } => {
                let mut tokens = vec![keyword!("elif ")];
                tokens.extend(self.lazy_expr_tokens(condition, text, ExprTokenConfig::branch()));
                tokens
            }
            LazyBranchKind::Else => vec![keyword!("else ")],
        }
    }

    pub fn lazy_branch_subtraces(
        &self,
        parent: &Trace,
        branch: &LazyBranch,
        trace_allocator: &TraceFactory,
        text: &Text,
    ) -> Arc<Vec<Arc<Trace>>> {
        // let mut subtraces = vec![];
        // match branch.kind {
        //     feature::LazyBranchKind::If { ref condition } => {
        //         subtraces.push(trace_allocator.new_trace(
        //             Some(parent),
        //             parent_indent + 1,
        //             TraceKind::Condition(condition.clone()),
        //         ))
        //     }
        //     feature::LazyBranchKind::Elif { ref condition } => {
        //         subtraces.push(trace_allocator.new_trace(
        //             Some(parent),
        //             parent_indent + 1,
        //             TraceKind::Condition(condition.clone()),
        //         ))
        //     }
        //     feature::LazyBranchKind::Else => (),
        // }
        // subtraces.extend();
        // Arc::new(subtraces)
        Arc::new(self.lazy_block_subtraces(parent, &branch.block, text))
    }
}
