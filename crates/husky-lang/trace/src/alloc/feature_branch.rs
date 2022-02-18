use feature::FeatureBranch;

use crate::*;

impl TraceAllocator {
    pub fn feature_branch_trace(
        &self,
        parent: &Trace,
        indent: Indent,
        branch: Arc<FeatureBranch>,
        text: &Text,
    ) -> Arc<Trace> {
        self.new_trace(Some(parent), indent, TraceKind::FeatureBranch(branch), text)
    }

    pub fn feature_branch_subtraces(
        &self,
        parent: &Trace,
        branch: &FeatureBranch,
        trace_allocator: &TraceAllocator,
        text: &Text,
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
        Arc::new(self.feature_block_subtraces(parent, &branch.block, text))
    }
}
