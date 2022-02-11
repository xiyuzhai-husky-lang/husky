use feature::FeatureBranch;

use crate::*;

impl TraceAllocator {
    pub fn feature_branch_trace(
        &self,
        parent: &Trace,
        indent: Indent,
        branch: Arc<FeatureBranch>,
    ) -> Arc<Trace> {
        self.new_trace(Some(parent), indent, TraceKind::FeatureBranch(branch))
    }

    pub fn feature_branch_subtraces(
        &self,
        parent: &Trace,
        parent_indent: Indent,
        branch: &FeatureBranch,
        trace_allocator: &TraceAllocator,
    ) -> Arc<Vec<Arc<Trace>>> {
        // let mut subtraces = vec![];
        // match branch.kind {
        //     feature::FeatureBranchKind::If { ref condition } => {
        //         subtraces.push(trace_allocator.new_trace(
        //             Some(parent),
        //             parent_indent + 1,
        //             TraceKind::Condition(condition.clone()),
        //         ))
        //     }
        //     feature::FeatureBranchKind::Elif { ref condition } => {
        //         subtraces.push(trace_allocator.new_trace(
        //             Some(parent),
        //             parent_indent + 1,
        //             TraceKind::Condition(condition.clone()),
        //         ))
        //     }
        //     feature::FeatureBranchKind::Else => (),
        // }
        // subtraces.extend();
        // Arc::new(subtraces)
        Arc::new(self.feature_block_subtraces(parent, &branch.block))
    }

    pub(crate) fn feature_branch_tokens(&self, branch: &FeatureBranch) -> Vec<TokenProps> {
        match branch.kind {
            feature::FeatureBranchKind::If { ref condition } => {
                let mut tokens = vec![keyword!("if ")];
                tokens.extend(self.feature_expr_tokens(condition, false));
                tokens
            }
            feature::FeatureBranchKind::Elif { ref condition } => {
                let mut tokens = vec![keyword!("elif ")];
                tokens.extend(self.feature_expr_tokens(condition, false));
                tokens
            }
            feature::FeatureBranchKind::Else => vec![keyword!("else ")],
        }
    }
}
