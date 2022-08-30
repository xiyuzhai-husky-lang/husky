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
        new_figure_canvases: Vec<(FigureCanvasKey, FigureCanvasData)>,
        new_figure_controls: Vec<(FigureControlKey, FigureControlData)>,
    },
    ActivateWithError {
        sample_id: SampleId,
        error: String,
    },
    TogglePin {
        new_figure_canvases: Vec<(FigureCanvasKey, FigureCanvasData)>,
        new_figure_controls: Vec<(FigureControlKey, FigureControlData)>,
    },
    TogglePinWithError {
        sample_id: SampleId,
        error: String,
    },
    ToggleExpansion {
        subtrace_ids: Vec<TraceId>,
        new_traces: Vec<TraceNodeData>,
        trace_stalks: Vec<(TraceStalkKey, TraceStalk)>,
        trace_stats: Vec<(TraceStatsKey, Option<TraceStats>)>,
    },
    ToggleShow {
        trace_id: TraceId,
    },
    Trace {
        trace_props: TraceData,
    },
    SetRestriction {
        new_figure_canvases: Vec<(FigureCanvasKey, FigureCanvasData)>,
        new_figure_controls: Vec<(FigureControlKey, FigureControlData)>,
        new_trace_stalks: Vec<(TraceStalkKey, TraceStalk)>,
        new_trace_statss: Vec<(TraceStatsKey, Option<TraceStats>)>,
    },
    SetRestrictionWithError {
        sample_id: SampleId,
        error: String,
    },
    TraceStalk {
        stalk: TraceStalk,
    },
}
