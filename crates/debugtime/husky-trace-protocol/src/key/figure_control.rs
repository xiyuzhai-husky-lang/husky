use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq)]
pub enum FigureControlKey {
    LoopFrame { parent: TraceId },
    Other { trace_id: TraceId, specific: bool },
}

impl FigureControlKey {
    pub fn from_trace_data(
        trace_data: &TraceData,
        presentation: &Presentation,
    ) -> FigureControlKey {
        Self::new(
            trace_data.opt_parent_id,
            trace_data.kind,
            trace_data.id,
            presentation,
        )
    }

    pub fn new(
        opt_parent_id: Option<TraceId>,
        trace_kind: TraceKind,
        trace_id: TraceId,
        presentation: &Presentation,
    ) -> FigureControlKey {
        match trace_kind {
            TraceKind::LoopFrame => FigureControlKey::LoopFrame {
                parent: opt_parent_id.unwrap(),
            },
            _ => FigureControlKey::Other {
                trace_id,
                specific: presentation.opt_sample_id().is_some(),
            },
        }
    }
}
