use feature::FeatureBranch;

use crate::*;

pub fn eval_feature_branch_trace_tokens(branch: &FeatureBranch) -> Vec<TokenProps> {
    match branch.kind {
        feature::FeatureBranchKind::If { ref condition } => {
            let mut tokens = vec![keyword!("if")];
            tokens.extend(eval_feature_expr_trace_tokens(condition));
            tokens
        }
        feature::FeatureBranchKind::Elif { ref condition } => todo!(),
        feature::FeatureBranchKind::Else => vec![keyword!("else")],
    }
}

pub fn eval_feature_branch_subtraces(
    parent: usize,
    indent: Indent,
    branch: &FeatureBranch,
) -> Arc<Vec<Arc<Trace>>> {
    todo!()
}
