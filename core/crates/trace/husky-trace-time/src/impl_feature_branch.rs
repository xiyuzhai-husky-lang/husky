use crate::*;

use super::impl_expr::ExprTokenConfig;

impl HuskyTraceTime {
    pub fn feature_branch_trace(
        &mut self,
        parent: &Trace,
        indent: Indent,
        branch: Arc<FeatureBranch>,
        text: &Text,
    ) -> Arc<Trace> {
        self.new_trace(
            Some(parent.id()),
            indent,
            TraceVariant::FeatureBranch(branch),
            text,
        )
    }

    pub(crate) fn feature_branch_lines(
        &mut self,
        indent: Indent,
        branch: &FeatureBranch,
        text: &Text,
    ) -> Vec<TraceLineData> {
        vec![TraceLineData {
            idx: 0,
            indent,
            tokens: self.feature_branch_tokens(branch, text),
        }]
    }

    pub(crate) fn feature_branch_tokens(
        &mut self,
        branch: &FeatureBranch,
        text: &Text,
    ) -> Vec<TraceTokenData> {
        match branch.variant {
            FeatureBranchVariant::If { ref condition } => {
                let mut tokens = vec![keyword!("if ")];
                tokens.extend(self.feature_expr_tokens(condition, text, ExprTokenConfig::branch()));
                tokens
            }
            FeatureBranchVariant::Elif { ref condition } => {
                let mut tokens = vec![keyword!("elif ")];
                tokens.extend(self.feature_expr_tokens(condition, text, ExprTokenConfig::branch()));
                tokens
            }
            FeatureBranchVariant::Else => vec![keyword!("else ")],
        }
    }

    pub fn feature_branch_subtraces(
        &mut self,
        parent: &Trace,
        branch: &FeatureBranch,
        trace_allocator: &HuskyTraceTime,
        text: &Text,
    ) -> Arc<Vec<Arc<Trace>>> {
        // let mut subtraces = vec![];
        // match branch.kind {
        //     feature_gen::FeatureBranchKind::If { ref condition } => {
        //         subtraces.push(trace_allocator.new_trace(
        //             Some(parent),
        //             parent_indent + 1,
        //             TraceKind::Condition(condition.clone()),
        //         ))
        //     }
        //     feature_gen::FeatureBranchKind::Elif { ref condition } => {
        //         subtraces.push(trace_allocator.new_trace(
        //             Some(parent),
        //             parent_indent + 1,
        //             TraceKind::Condition(condition.clone()),
        //         ))
        //     }
        //     feature_gen::FeatureBranchKind::Else => (),
        // }
        // subtraces.extend();
        // Arc::new(subtraces)
        Arc::new(self.feature_lazy_block_subtraces(parent, &branch.block, text))
    }
}
