use crate::*;

impl TraceVariant {
    pub fn opt_arrival_indicator(&self) -> Option<&ValDomain> {
        match self {
            TraceVariant::Main(_) => None,
            TraceVariant::Module { .. } => None,
            TraceVariant::EntityFeature { .. } => None,
            TraceVariant::FeatureStmt(stmt) => stmt.opt_arrival_indicator.as_ref(),
            TraceVariant::FeatureBranch(branch) => branch.opt_arrival_indicator.as_ref(),
            TraceVariant::FeatureExpr(expr) => expr.opt_arrival_indicator.as_ref(),
            TraceVariant::FeatureCallArgument { .. } => None,
            TraceVariant::FuncStmt { .. } => None, // could be changed in the future
            TraceVariant::ProcStmt { .. } => None, // could be changed in the future
            TraceVariant::ProcBranch { .. } => None, // could be changed in the future
            TraceVariant::FuncBranch { .. } => None, // could be changed in the future
            TraceVariant::LoopFrame { .. } => None, // could be changed in the future
            TraceVariant::EagerExpr { .. } => None, // could be changed in the future
            TraceVariant::EagerCallArgument { .. } => None,
            TraceVariant::CallHead { .. } => None,
        }
    }
}
