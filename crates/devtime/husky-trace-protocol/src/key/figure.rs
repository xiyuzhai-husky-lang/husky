use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct GenericFigureCanvasKey {
    trace_id: TraceId,
    partitions: Partitions,
    restriction: Option<Restriction>,
}

pub type GenericFigureCanvasMap = HashMap<GenericFigureCanvasKey, GenericFigureCanvasData>;

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct SpecificFigureCanvasKey {
    trace_id: TraceId,
    sample_id: SampleId,
}

impl GenericFigureCanvasKey {
    pub fn from_trace_data(trace_data: &TraceData, presentation: &Presentation) -> Option<Self> {
        Self::new(trace_data.kind, trace_data.id, presentation)
    }

    pub fn new(
        trace_kind: TraceKind,
        trace_id: TraceId,
        presentation: &Presentation,
    ) -> Option<Self> {
        match trace_kind {
            TraceKind::Main
            | TraceKind::EntityFeatureLazy
            | TraceKind::EntityFeatureEager
            | TraceKind::FeatureStmt
            | TraceKind::FeatureBranch
            | TraceKind::FeatureExprLazy
            | TraceKind::FeatureExprEager
            | TraceKind::FeatureCallArgument => Some(GenericFigureCanvasKey {
                trace_id,
                partitions: presentation.partitions().clone(),
                restriction: presentation.restriction(),
            }),
            TraceKind::FuncStmt
            | TraceKind::EagerStmt
            | TraceKind::FuncBranch
            | TraceKind::EagerBranch
            | TraceKind::EagerExpr
            | TraceKind::LoopFrame
            | TraceKind::Module
            | TraceKind::CallHead
            | TraceKind::EagerCallArgument => None,
        }
    }
}

impl SpecificFigureCanvasKey {
    pub fn from_trace_data(trace_data: &TraceData, presentation: &Presentation) -> Option<Self> {
        Self::new(trace_data.kind, trace_data.id, presentation)
    }

    pub fn new(
        trace_kind: TraceKind,
        trace_id: TraceId,
        presentation: &Presentation,
    ) -> Option<Self> {
        match trace_kind {
            TraceKind::Main
            | TraceKind::EntityFeatureLazy
            | TraceKind::EntityFeatureEager
            | TraceKind::FeatureStmt
            | TraceKind::FeatureBranch
            | TraceKind::FeatureExprLazy
            | TraceKind::FeatureExprEager
            | TraceKind::FeatureCallArgument
            | TraceKind::FuncStmt
            | TraceKind::EagerStmt
            | TraceKind::FuncBranch
            | TraceKind::EagerBranch
            | TraceKind::EagerExpr
            | TraceKind::LoopFrame
            | TraceKind::EagerCallArgument => Some(SpecificFigureCanvasKey {
                trace_id,
                sample_id: presentation.sample_id(),
            }),
            TraceKind::Module | TraceKind::CallHead => None,
        }
    }
}
