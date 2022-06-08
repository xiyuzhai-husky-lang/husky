use focus::Focus;
use json_result::JsonResult;
use runtime_db::FigureControlProps;

use super::*;
use crate::*;

#[derive(Debug, Serialize, Clone)]
pub(super) struct Response<'a> {
    pub request_id: usize,
    pub variant: ResponseVariant<'a>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(tag = "kind")]
pub(super) enum ResponseVariant<'a> {
    Init {
        init_state: InitState<'a>,
    },
    Activate {
        figure_props: FigureProps,
        figure_control_props: FigureControlProps,
    },
    ToggleExpansion {
        effective_opt_input_id: Option<usize>,
        subtraces: Avec<Trace<'static>>,
        associated_traces: Vec<Arc<Trace<'static>>>,
    },
    ToggleShow {
        id: TraceId,
    },
    Trace {
        id: TraceId,
        trace: Arc<Trace<'static>>,
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
        stalk: Arc<TraceStalk<'static>>,
    },
}
