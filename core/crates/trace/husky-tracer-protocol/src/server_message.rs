use super::{trace::TraceRawData, *};

pub type JsonResult<T> = Result<T, String>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HuskyTracerServerMessage {
    pub opt_request_id: Option<usize>,
    pub variant: HuskyTracerServerMessageVariant,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "kind")]
pub enum HuskyTracerServerMessageVariant {
    Init {
        init_data: InitData,
    },
    Activate {
        figure_canvas_data: FigureCanvasData,
        figure_control_data: FigureControlData,
    },
    ActivateWithError {
        sample_id: usize,
        error: String,
    },
    ToggleExpansion {
        subtrace_ids: Vec<TraceId>,
        new_traces: Vec<TraceNodeData>,
        trace_stalks: Vec<(TraceStalkKey, TraceStalkRawData)>,
    },
    ToggleShow {
        trace_id: TraceId,
    },
    Trace {
        trace_props: TraceRawData,
    },
    LockFocus {
        figure_canvas_data: FigureCanvasData,
        figure_control_data: FigureControlData,
    },
    LockFocusWithError {
        sample_id: usize,
        error: String,
    },
    TraceStalk {
        stalk: TraceStalkRawData,
    },
}
