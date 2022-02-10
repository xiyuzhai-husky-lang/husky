use feature::FeatureBranch;

use crate::*;

impl TraceAllocator {
    pub fn feature_branch_subtraces(
        &self,
        parent: usize,
        indent: Indent,
        branch: &FeatureBranch,
        trace_allocator: &TraceAllocator,
    ) -> Arc<Vec<Arc<Trace>>> {
        let mut subtraces = vec![];
        match branch.kind {
            feature::FeatureBranchKind::If { ref condition } => {
                subtraces.push(trace_allocator.new_trace(
                    Some(parent),
                    indent + 2,
                    TraceKind::Condition(condition.clone()),
                ))
            }
            feature::FeatureBranchKind::Elif { ref condition } => {
                subtraces.push(trace_allocator.new_trace(
                    Some(parent),
                    indent + 2,
                    TraceKind::Condition(condition.clone()),
                ))
            }
            feature::FeatureBranchKind::Else => (),
        }
        subtraces.extend(self.feature_block_subtraces(Some(parent), &branch.block));
        Arc::new(subtraces)
    }
}
