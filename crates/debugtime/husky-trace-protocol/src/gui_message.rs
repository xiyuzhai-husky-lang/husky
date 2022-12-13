use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(tag = "kind")]
pub struct HuskyTracerGuiMessage {
    pub opt_request_id: Option<usize>,
    pub variant: HuskyTracerGuiMessageVariant,
}

#[cfg(not(feature = "verify_consistency"))]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(tag = "kind")]
pub enum HuskyTracerGuiMessageVariant {
    HotReloadRequest,
    Activate {
        trace_id: TraceId,
        needs_figure_canvases: bool,
        needs_figure_controls: bool,
    },
    ToggleExpansion {
        trace_id: TraceId,
    },
    ToggleShow {
        trace_id: TraceId,
    },
    TogglePin {
        trace_id: TraceId,
        needs_figure_canvases: bool,
        needs_figure_controls: bool,
    },
    // Trace {
    //     id: TraceId,
    // },
    SetPresentation {
        presentation: Presentation,
        needs_figure_canvases: bool,
        needs_figure_controls: bool,
        needs_stalks: bool,
        needs_statss: bool,
    },
    // TraceStalk {
    //     trace_id: TraceId,
    // },
    UpdateFigureControlData {
        trace_id: TraceId,
        figure_control_data: FigureControlData,
    },
}
