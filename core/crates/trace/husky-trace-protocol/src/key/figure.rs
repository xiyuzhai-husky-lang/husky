use vec_like::VecSet;

use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub enum FigureCanvasKey {
    Null,
    Generic {
        trace_id: TraceId,
        partitions: Vec<PartitionDefnData>,
        arrivals: VecSet<TraceId>,
        enters: VecSet<TraceId>,
    },
    Specific {
        trace_id: TraceId,
    },
}

impl FigureCanvasKey {
    pub fn from_trace_data(trace_data: &TraceData, restriction: &Restriction) -> FigureCanvasKey {
        Self::new(trace_data.kind, trace_data.id, restriction)
    }

    pub fn new(
        trace_kind: TraceKind,
        trace_id: TraceId,
        restriction: &Restriction,
    ) -> FigureCanvasKey {
        match trace_kind {
            TraceKind::Main
            | TraceKind::FeatureStmt
            | TraceKind::FeatureBranch
            | TraceKind::FeatureExpr
            | TraceKind::FeatureCallArgument => match restriction {
                Restriction::Specific { sample_id } => todo!(),
                Restriction::Generic {
                    partitions,
                    arrivals,
                    enters,
                } => FigureCanvasKey::Generic {
                    trace_id,
                    partitions: partitions.clone(),
                    enters: enters.clone(),
                    arrivals: arrivals.clone(),
                },
            },
            TraceKind::FuncStmt
            | TraceKind::ProcStmt
            | TraceKind::FuncBranch
            | TraceKind::ProcBranch
            | TraceKind::EagerExpr
            | TraceKind::LoopFrame => FigureCanvasKey::Specific { trace_id },
            TraceKind::CallHead | TraceKind::EagerCallArgument => FigureCanvasKey::Null,
        }
    }
}
