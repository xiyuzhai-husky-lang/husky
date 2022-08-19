use vec_like::VecSet;

use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub enum FigureCanvasKey {
    Null,
    Generic {
        trace_id: TraceId,
        partitions: Partitions,
        arrivals: Arrivals,
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
            | TraceKind::EntityFeatureLazy
            | TraceKind::EntityFeatureEager
            | TraceKind::FeatureStmt
            | TraceKind::FeatureBranch
            | TraceKind::FeatureExprLazy
            | TraceKind::FeatureExprEager
            | TraceKind::FeatureCallArgument => {
                if restriction.is_specific() {
                    FigureCanvasKey::Specific { trace_id }
                } else {
                    FigureCanvasKey::Generic {
                        trace_id,
                        partitions: restriction.partitions().clone(),
                        enters: restriction.enters().clone(),
                        arrivals: restriction.arrivals().clone(),
                    }
                }
            }
            TraceKind::FuncStmt
            | TraceKind::ProcStmt
            | TraceKind::FuncBranch
            | TraceKind::ProcBranch
            | TraceKind::EagerExpr
            | TraceKind::LoopFrame => FigureCanvasKey::Specific { trace_id },
            TraceKind::Module | TraceKind::CallHead | TraceKind::EagerCallArgument => {
                FigureCanvasKey::Null
            }
        }
    }
}
