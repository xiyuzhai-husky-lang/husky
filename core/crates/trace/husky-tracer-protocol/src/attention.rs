mod label;
mod partition;

pub use label::*;
pub use partition::*;
use sycamore::prelude::Signalable;

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum Attention {
    Specific {
        sample_id: usize,
    },
    Generic {
        partitions: Vec<PartitionDefnData>,
        constraints: Vec<Constraint>,
    },
}

impl Signalable for Attention {}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum Constraint {}

impl Default for Attention {
    fn default() -> Self {
        Attention::Generic {
            partitions: vec![PartitionDefnData {
                name: "other".into(),
                ncol: 7,
                variant: PartitionDefnDataVariant::Other,
            }],
            constraints: vec![],
        }
    }
}

impl Attention {
    pub fn has_stalk(&self, trace_kind: TraceKind) -> bool {
        match trace_kind {
            TraceKind::Main
            | TraceKind::FeatureStmt
            | TraceKind::FeatureBranch
            | TraceKind::FeatureExpr => match self {
                Attention::Specific { .. } => true,
                Attention::Generic { .. } => false,
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

    pub fn opt_sample_id(&self) -> Option<usize> {
        match self {
            Attention::Specific {
                sample_id: input_id,
            } => Some(*input_id),
            Attention::Generic { .. } => None,
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
