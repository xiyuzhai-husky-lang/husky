use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum Focus {
    Specific { input_id: usize },
    Generic {},
}

impl Default for Focus {
    fn default() -> Self {
        Focus::Generic {}
    }
}

impl Focus {
    pub fn has_stalk(&self, trace_kind: TraceKind) -> bool {
        match trace_kind {
            TraceKind::Main
            | TraceKind::FeatureStmt
            | TraceKind::FeatureBranch
            | TraceKind::FeatureExpr => match self {
                Focus::Specific { .. } => true,
                Focus::Generic {} => false,
            },
            TraceKind::FeatureCallInput
            | TraceKind::FuncStmt
            | TraceKind::ProcStmt
            | TraceKind::ProcBranch
            | TraceKind::LoopFrame
            | TraceKind::EagerExpr
            | TraceKind::CallHead => false,
        }
    }

    pub fn opt_input_id(&self) -> Option<usize> {
        match self {
            Focus::Specific { input_id } => Some(*input_id),
            Focus::Generic {} => None,
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
