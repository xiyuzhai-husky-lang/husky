use super::{trace::TraceProps, *};
use crate::*;

pub type JsonResult<T> = Result<T, String>;

#[derive(Debug, Serialize, Clone)]
pub struct Response {
    pub request_id: usize,
    pub variant: ResponseVariant,
}

#[derive(Debug, Serialize, Clone)]
#[serde(tag = "kind")]
pub enum ResponseVariant {
    Init {
        init_state: InitState,
    },
    Activate {
        figure_props: FigureProps,
        figure_control_props: FigureControlProps,
    },
    ToggleExpansion {
        effective_opt_input_id: Option<usize>,
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
