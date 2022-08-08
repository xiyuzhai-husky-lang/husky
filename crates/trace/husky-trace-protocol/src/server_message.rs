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
        #[serde(skip_serializing_if = "Option::is_none")]
        opt_figure_canvas_data: Option<FigureCanvasData>,
        #[serde(skip_serializing_if = "Option::is_none")]
        opt_figure_control_data: Option<FigureControlData>,
    },
    ActivateWithError {
        sample_id: SampleId,
        error: String,
    },
    TogglePin {
        #[serde(skip_serializing_if = "Option::is_none")]
        opt_figure_canvas_data: Option<FigureCanvasData>,
        #[serde(skip_serializing_if = "Option::is_none")]
        opt_figure_control_data: Option<FigureControlData>,
    },
    TogglePinWithError {
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
    SetRestriction {
        opt_figure_canvas_data: Option<FigureCanvasData>,
        opt_figure_control_data: Option<FigureControlData>,
        new_trace_stalks: Vec<(TraceStalkKey, TraceStalkData)>,
    },
    SetRestrictionWithError {
        sample_id: SampleId,
        error: String,
    },
    TraceStalk {
        stalk: TraceStalkData,
    },
}
