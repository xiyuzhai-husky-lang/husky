use focus::Focus;
use json_result::JsonResult;

use super::*;
use crate::*;

#[derive(Debug, Serialize, Clone)]
#[serde(tag = "kind")]
pub(super) enum Response<'a> {
    Init {
        init_state: InitData<'a>,
    },
    Activate {
        id: TraceId,
        opt_focus_for_figure: Option<Focus>,
        opt_figure: Option<FigureProps>,
        opt_figure_control: Option<FigureControl>,
    },
    ToggleExpansion {
        id: TraceId,
        effective_opt_input_id: Option<usize>,
        opt_subtraces: Option<Avec<Trace<'static>>>,
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
    },
    TraceStalk {
        trace_id: TraceId,
        input_id: usize,
        stalk: Arc<TraceStalk<'static>>,
    },
}
