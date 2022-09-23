mod hot_reload;
mod update;

pub use hot_reload::*;
use trackable::{Trackable, TrackableAtom, TrackableMap, TrackableVec};
pub use update::*;

use crate::*;

#[derive(Default)]
pub struct HuskyDebugtimeState {
    pub(crate) restriction: TrackableAtom<Restriction>,
    pub(crate) pins: VecSet<TraceId>,
    pub(crate) opt_active_trace_id: TrackableAtom<Option<TraceId>>,
    pub(crate) trace_nodes: TrackableVec<Option<TraceNode>>,
    pub(crate) figure_canvases: TrackableMap<FigureCanvasKey, FigureCanvasData>,
    pub(crate) figure_controls: TrackableMap<FigureControlKey, FigureControlData>,
    pub(crate) trace_stalks: TrackableMap<TraceStalkKey, TraceStalk>,
    pub(crate) trace_statss: TrackableMap<TraceStatsKey, Option<TraceStats>>,
    root_traces: Vec<TraceId>,
    pub(crate) subtrace_ids_map: HashMap<SubtracesKey, Vec<TraceId>>,
}

pub struct DebugtimeStateChange {
    pub(crate) restriction: <TrackableAtom<Restriction> as Trackable>::Change,
    pub(crate) opt_active_trace_id: <TrackableAtom<Option<TraceId>> as Trackable>::Change,
    pub(crate) trace_nodes: <TrackableVec<Option<TraceNode>> as Trackable>::Change,
    pub(crate) figure_canvases:
        <TrackableMap<FigureCanvasKey, FigureCanvasData> as Trackable>::Change,
    pub(crate) figure_controls:
        <TrackableMap<FigureControlKey, FigureControlData> as Trackable>::Change,
    pub(crate) trace_stalks: <TrackableMap<TraceStalkKey, TraceStalk> as Trackable>::Change,
    pub(crate) trace_statss: <TrackableMap<TraceStatsKey, Option<TraceStats>> as Trackable>::Change,
    root_traces: Vec<TraceId>,
    pub(crate) subtrace_ids_map: HashMap<SubtracesKey, Vec<TraceId>>,
}

// implementation details

impl Trackable for HuskyDebugtimeState {
    type Change = DebugtimeStateChange;

    fn take_change(&mut self) -> trackable::TrackableTakeChangeM<Self> {
        todo!()
    }
}

impl HuskyDebugtimeState {
    pub(crate) fn root_traces(&self) -> &[TraceId] {
        &self.root_traces
    }

    pub(crate) fn set_root_traces(&mut self, root_traces: Vec<TraceId>) {
        self.root_traces = root_traces
    }
}
