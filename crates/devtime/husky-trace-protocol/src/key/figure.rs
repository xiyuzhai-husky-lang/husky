use vec_like::VecSet;

use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub enum FigureCanvasKey {
    Null,
    Generic {
        trace_id: TraceId,
        partitions: Partitions,
        restriction: Restriction,
    },
    Specific {
        trace_id: TraceId,
        sample_id: SampleId,
    },
}

impl FigureCanvasKey {
    pub fn from_trace_data(
        trace_data: &TraceData,
        restriction: &Presentation,
        is_specific: bool,
    ) -> FigureCanvasKey {
        Self::new(trace_data.kind, trace_data.id, restriction, is_specific)
    }

    pub fn new(
        trace_kind: TraceKind,
        trace_id: TraceId,
        presentation: &Presentation,
        is_specific: bool,
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
                if is_specific {
                    FigureCanvasKey::Specific {
                        trace_id,
                        sample_id: presentation.sample_id(),
                    }
                } else {
                    FigureCanvasKey::Generic {
                        trace_id,
                        partitions: presentation.partitions().clone(),
                        restriction: presentation.restriction(),
                    }
                }
            }
            TraceKind::FuncStmt
            | TraceKind::ProcStmt
            | TraceKind::FuncBranch
            | TraceKind::ProcBranch
            | TraceKind::EagerExpr
            | TraceKind::LoopFrame => FigureCanvasKey::Specific {
                trace_id,
                sample_id: presentation.sample_id(),
            },
            TraceKind::Module | TraceKind::CallHead | TraceKind::EagerCallArgument => {
                FigureCanvasKey::Null
            }
        }
    }
}
