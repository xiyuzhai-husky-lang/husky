use trace::TraceAllocator;

use super::*;
use crate::*;

#[derive(Debug, Serialize, Clone)]
#[serde(tag = "type")]
pub(super) enum Response<'a> {
    Init {
        active_trace_id: Option<TraceId>,
        opt_input_id: Option<usize>,
        traces: &'a TraceAllocator,
        root_traces: &'a [Arc<Trace>],
        expansions: &'a HashMap<TraceId, bool>,
        showns: &'a HashMap<TraceId, bool>,
    },
    Subtraces {
        id: TraceId,
        input_locked_on: Option<usize>,
        subtraces: Arc<Vec<Arc<Trace>>>,
    },
    Figure {
        id: TraceId,
        figure: Option<FigureProps>,
    },
    DidActivate {
        id: TraceId,
    },
    DidToggleExpansion {
        id: TraceId,
    },
    DidToggleShow {
        id: TraceId,
    },
    Trace {
        id: TraceId,
        trace: Arc<Trace>,
    },
    DidLockInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        input_locked_on: Option<Option<usize>>,
        message: Option<String>,
    },
    TraceStalk {
        trace_id: TraceId,
        input_id: usize,
        stalk: Arc<TraceStalk>,
    },
}
