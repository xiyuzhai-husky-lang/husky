mod hot_reload;
mod update;

pub use hot_reload::*;
use trackable::{
    TrackSimple, Trackable, TrackableAtom, TrackableMakeChangeM, TrackableMap,
    TrackableTakeChangeM, TrackableVec, TrackableVecSimple,
};
pub use update::*;

use crate::*;

#[derive(Default)]
pub struct HuskyDebugtimeState {
    pub(crate) restriction: TrackableAtom<Restriction>,
    pub(crate) pins: VecSet<TraceId>,
    pub(crate) opt_active_trace_id: TrackableAtom<Option<TraceId>>,
    pub(crate) trace_nodes: TrackableVec<TraceNode>,
    pub(crate) figure_canvases: TrackableMap<FigureCanvasKey, FigureCanvasData>,
    pub(crate) figure_controls: TrackableMap<FigureControlKey, FigureControlData>,
    pub(crate) trace_stalks: TrackableMap<TraceStalkKey, TraceStalk>,
    pub(crate) trace_statss: TrackableMap<TraceStatsKey, Option<TraceStats>>,
    root_traces: TrackableVecSimple<TraceId>,
    pub(crate) subtrace_ids_map: TrackableMap<SubtracesKey, Vec<TraceId>>,
}

pub struct DebugtimeStateChange {
    pub(crate) restriction: <TrackableAtom<Restriction> as Trackable>::Change,
    pub(crate) opt_active_trace_id: <TrackableAtom<Option<TraceId>> as Trackable>::Change,
    pub(crate) trace_nodes: <TrackableVec<TraceNode> as Trackable>::Change,
    pub(crate) figure_canvases:
        <TrackableMap<FigureCanvasKey, FigureCanvasData> as Trackable>::Change,
    pub(crate) figure_controls:
        <TrackableMap<FigureControlKey, FigureControlData> as Trackable>::Change,
    pub(crate) trace_stalks: <TrackableMap<TraceStalkKey, TraceStalk> as Trackable>::Change,
    pub(crate) trace_statss: <TrackableMap<TraceStatsKey, Option<TraceStats>> as Trackable>::Change,
    root_traces: <TrackableVecSimple<TraceId> as Trackable>::Change,
    pub(crate) subtrace_ids_map: <TrackableMap<SubtracesKey, Vec<TraceId>> as Trackable>::Change,
}

// implementation details

impl Trackable for HuskyDebugtimeState {
    type Change = DebugtimeStateChange;

    fn take_change(&mut self) -> TrackableTakeChangeM<Self> {
        TrackableTakeChangeM::Ok(DebugtimeStateChange {
            restriction: self.restriction.take_change()?,
            opt_active_trace_id: self.opt_active_trace_id.take_change()?,
            trace_nodes: self.trace_nodes.take_change()?,
            figure_canvases: self.figure_canvases.take_change()?,
            figure_controls: self.figure_controls.take_change()?,
            trace_stalks: self.trace_stalks.take_change()?,
            trace_statss: self.trace_statss.take_change()?,
            root_traces: self.root_traces.take_change()?,
            subtrace_ids_map: self.subtrace_ids_map.take_change()?,
        })
    }
}

impl HuskyDebugtimeState {
    pub(crate) fn root_traces(&self) -> &[TrackSimple<TraceId>] {
        &self.root_traces
    }

    pub(crate) fn set_root_traces(
        &mut self,
        root_traces: Vec<TraceId>,
    ) -> TrackableMakeChangeM<Self, ()> {
        self.root_traces
            .set(root_traces.into_iter().map(|id| id.into()).collect())?;
        TrackableMakeChangeM::default()
    }
}
