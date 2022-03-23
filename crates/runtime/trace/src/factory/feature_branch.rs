use crate::*;

use super::expr::ExprTokenConfig;

impl<'eval> TraceFactory<'eval> {
    pub fn feature_branch_trace(
        &self,
        parent: &Trace,
        indent: Indent,
        branch: Arc<FeatureBranch>,
        text: &Text,
    ) -> Arc<Trace<'eval>> {
        self.new_trace(
            Some(parent.id),
            indent,
            TraceKind::FeatureBranch(branch),
            text,
        )
    }

    pub(super) fn feature_branch_lines(
        &self,
        indent: Indent,
        branch: &FeatureBranch,
        text: &Text,
    ) -> Vec<LineProps<'eval>> {
        vec![LineProps {
            idx: 0,
            indent,
            tokens: self.feature_branch_tokens(branch, text),
        }]
    }

    pub(super) fn feature_branch_tokens(
        &self,
        branch: &FeatureBranch,
        text: &Text,
    ) -> Vec<TokenProps<'eval>> {
        match branch.kind {
            FeatureBranchKind::If { ref condition } => {
                let mut tokens = vec![keyword!("if ")];
                tokens.extend(self.feature_expr_tokens(condition, text, ExprTokenConfig::branch()));
                tokens
            }
            FeatureBranchKind::Elif { ref condition } => {
                let mut tokens = vec![keyword!("elif ")];
                tokens.extend(self.feature_expr_tokens(condition, text, ExprTokenConfig::branch()));
                tokens
            }
            FeatureBranchKind::Else => vec![keyword!("else ")],
        }
    }

    pub fn feature_branch_subtraces(
        &self,
        parent: &Trace<'eval>,
        branch: &FeatureBranch,
        trace_allocator: &TraceFactory<'eval>,
        text: &Text,
    ) -> Arc<Vec<Arc<Trace<'eval>>>> {
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
        Arc::new(self.feature_block_subtraces(parent, &branch.block, text))
    }
}
