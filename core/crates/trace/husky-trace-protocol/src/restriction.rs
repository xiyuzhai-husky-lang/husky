mod label;
mod partition;

pub use label::*;
pub use partition::*;
use sycamore::prelude::Signalable;
use vec_like::VecSet;

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum Restriction {
    Specific {
        sample_id: SampleId,
    },
    Generic {
        partitions: Vec<PartitionDefnData>,
        arrivals: VecSet<TraceId>,
        enters: VecSet<TraceId>,
    },
}

impl Signalable for Restriction {}

impl Default for Restriction {
    fn default() -> Self {
        Restriction::Generic {
            partitions: vec![PartitionDefnData {
                name: "other".into(),
                ncol: 7,
                variant: PartitionDefnDataVariant::Other,
            }],
            arrivals: Default::default(),
            enters: Default::default(),
        }
    }
}

impl Restriction {
    pub fn has_stalk(&self, trace_kind: TraceKind) -> bool {
        match trace_kind {
            TraceKind::Main
            | TraceKind::FeatureStmt
            | TraceKind::FeatureBranch
            | TraceKind::FeatureExpr => match self {
                Restriction::Specific { .. } => true,
                Restriction::Generic { .. } => false,
            },
            TraceKind::FeatureCallArgument
            | TraceKind::EagerCallArgument
            | TraceKind::FuncStmt
            | TraceKind::ProcStmt
            | TraceKind::ProcBranch
            | TraceKind::FuncBranch
            | TraceKind::LoopFrame
            | TraceKind::EagerExpr
            | TraceKind::CallHead => false,
        }
    }

    pub fn opt_sample_id(&self) -> Option<SampleId> {
        match self {
            Restriction::Specific { sample_id, .. } => Some(*sample_id),
            Restriction::Generic { .. } => None,
        }
    }
}
// function tell_has_extra(trace: Trace, sample_id: number | null): boolean {
//     switch (trace.kind) {
//         case "Main":
//         case "FeatureStmt":
//         case "FeatureBranch":
//         case "FeatureExpr":
//             return sample_id !== null;
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
