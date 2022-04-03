use focus::Focus;

use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(tag = "kind")]
pub(super) enum Query {
    Activate {
        id: TraceId,
        opt_focus_for_figure: Option<Focus>,
    },
    ToggleExpansion {
        id: TraceId,
        effective_opt_input_id: Option<usize>,
        request_subtraces: bool,
    },
    ToggleShow {
        id: TraceId,
    },
    Trace {
        id: TraceId,
    },
    DecodeFocus {
        command: String,
    },
    LockFocus {
        focus: Focus,
        opt_figure_trace_id: Option<TraceId>,
    },
    TraceStalk {
        trace_id: TraceId,
        input_id: usize,
    },
}
