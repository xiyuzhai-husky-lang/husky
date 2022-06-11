use super::{trace::TraceProps, *};

pub type JsonResult<T> = Result<T, String>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DebuggerServerMessage {
    pub opt_request_id: Option<usize>,
    pub variant: DebuggerServerMessageVariant,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "kind")]
pub enum DebuggerServerMessageVariant {
    Init {
        init_data: InitData,
    },
    Activate {
        figure_props: FigureProps,
        figure_control_props: FigureControlProps,
    },
    ToggleExpansion {
        subtraces: Vec<TraceProps>,
        associated_traces: Vec<TraceId>,
    },
    ToggleShow {
        trace_id: TraceId,
    },
    Trace {
        trace_props: TraceProps,
    },
    DecodeFocus {
        focus_result: JsonResult<Focus>,
    },
    LockFocus {
        focus: Focus,
        opt_active_trace_id_for_figure: Option<TraceId>,
        opt_figure: Option<FigureProps>,
        opt_figure_control: Option<FigureControlProps>,
    },
    TraceStalk {
        stalk: TraceStalk,
    },
}
