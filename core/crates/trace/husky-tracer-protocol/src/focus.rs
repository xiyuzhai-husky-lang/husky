use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Focus {
    pub opt_input_id: Option<usize>,
}

impl Default for Focus {
    fn default() -> Self {
        Self { opt_input_id: None }
    }
}

impl Focus {
    pub fn has_stalk(&self, trace_kind: TraceKind) -> bool {
        match trace_kind {
            TraceKind::Main
            | TraceKind::FeatureStmt
            | TraceKind::FeatureBranch
            | TraceKind::FeatureExpr => self.opt_input_id.is_some(),
            TraceKind::FeatureCallInput
            | TraceKind::FuncStmt
            | TraceKind::ProcStmt
            | TraceKind::ProcBranch
            | TraceKind::LoopFrame
            | TraceKind::EagerExpr
            | TraceKind::CallHead => false,
        }
    }
}
// function tell_has_extra(trace: Trace, input_id: number | null): boolean {
//     switch (trace.kind) {
//         case "Main":
//         case "FeatureStmt":
//         case "FeatureBranch":
//         case "FeatureExpr":
//             return input_id !== null;
//         case "FuncStmt":
//         case "LoopFrame":
//         case "ProcStmt":
//         case "EagerExpr":
//         case "CallHead":
//         case "FeatureCallInput":
//         case "LoopFrame":
//         case "ProcBranch":
//             return false;
//     }
// }
