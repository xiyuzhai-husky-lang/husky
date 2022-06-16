use super::{trace::TraceData, *};

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
    ToggleExpansion {
        subtrace_ids: Vec<TraceId>,
        new_traces: Vec<TraceNodeData>,
    },
    ToggleShow {
        trace_id: TraceId,
    },
    Trace {
        trace_props: TraceData,
    },
    DecodeFocus {
        focus_result: JsonResult<Focus>,
    },
    LockFocus {
        figure_canvas: FigureCanvasData,
        figure_control: FigureControlData,
    },
    TraceStalk {
        stalk: TraceStalk,
    },
}
