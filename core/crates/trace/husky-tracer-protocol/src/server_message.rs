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
    ActivateWithError {
        sample_id: SampleId,
        error: String,
    },
    ToggleExpansion {
        subtrace_ids: Vec<TraceId>,
        new_traces: Vec<TraceNodeData>,
        trace_stalks: Vec<(TraceStalkKey, TraceStalkData)>,
    },
    ToggleShow {
        trace_id: TraceId,
    },
    Trace {
        trace_props: TraceData,
    },
    LockAttention {
        opt_figure_data: Option<(FigureCanvasData, FigureControlData)>,
        new_trace_stalks: Vec<(TraceStalkKey, TraceStalkData)>,
    },
    LockAttentionWithError {
        sample_id: SampleId,
        error: String,
    },
    TraceStalk {
        stalk: TraceStalkData,
    },
}
