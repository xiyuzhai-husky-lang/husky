use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub(super) enum Query {
    Subtraces {
        trace_id: TraceId,
        opt_input_id: Option<usize>,
    },
    Figure {
        id: TraceId,
    },
    Activate {
        id: TraceId,
    },
    ToggleExpansion {
        id: TraceId,
    },
    ToggleShow {
        id: TraceId,
    },
    Trace {
        id: TraceId,
    },
    LockInput {
        input_str: String,
    },
    TraceStalk {
        trace_id: TraceId,
        input_id: usize,
    },
}
