use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq)]
pub enum FigureControlKey {
    LoopFrame { parent: TraceId },
    Other { this: TraceId },
}

impl FigureControlKey {
    pub fn new(trace_props: &TraceProps) -> FigureControlKey {
        match trace_props.kind {
            TraceKind::LoopFrame => FigureControlKey::LoopFrame {
                parent: trace_props.opt_parent_id.unwrap(),
            },
            _ => FigureControlKey::Other {
                this: trace_props.id,
            },
        }
    }
}
