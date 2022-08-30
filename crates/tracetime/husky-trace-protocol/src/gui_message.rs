use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(tag = "kind")]
pub struct HuskyTracerGuiMessage {
    pub opt_request_id: Option<usize>,
    pub variant: HuskyTracerGuiMessageVariant,
}

#[cfg(feature = "verify_consistency")]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(tag = "kind")]
pub enum HuskyTracerGuiMessageVariant {
    InitDataRequest,
    Activate {
        trace_id: TraceId,
        needs_figure_canvas_data: bool,
        needs_figure_control_data: bool,
    },
    ToggleExpansion {
        trace_id: TraceId,
    },
    ToggleShow {
        trace_id: TraceId,
    },
    TogglePin {
        trace_id: TraceId,
        needs_figure_canvas_data: bool,
        needs_figure_control_data: bool,
    },
    Trace {
        id: TraceId,
    },
    SetRestriction {
        restriction: Restriction,
        needs_figure_canvas_data: bool,
        needs_figure_control_data: bool,
        new_stalk_keys: Vec<TraceStalkKey>,
        new_stats_keys: Vec<TraceStatsKey>,
    },
    TraceStalk {
        trace_id: TraceId,
    },
    UpdateFigureControlData {
        trace_id: TraceId,
        restriction: Restriction,
        figure_control_props: FigureControlData,
    },
}

#[cfg(not(feature = "verify_consistency"))]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(tag = "kind")]
pub enum HuskyTracerGuiMessageVariant {
    InitDataRequest,
    Activate {
        trace_id: TraceId,
        needs_figure_canvas_data: bool,
        needs_figure_control_data: bool,
    },
    ToggleExpansion {
        trace_id: TraceId,
    },
    ToggleShow {
        trace_id: TraceId,
    },
    TogglePin {
        trace_id: TraceId,
        needs_figure_canvas_data: bool,
        needs_figure_control_data: bool,
    },
    Trace {
        id: TraceId,
    },
    SetRestriction {
        restriction: Restriction,
        needs_figure_canvas_data: bool,
        needs_figure_control_data: bool,
        needs_stalks: bool,
        needs_statss: bool,
    },
    TraceStalk {
        trace_id: TraceId,
    },
    UpdateFigureControlData {
        trace_id: TraceId,
        figure_control_data: FigureControlData,
    },
}
