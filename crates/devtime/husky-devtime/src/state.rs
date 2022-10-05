use crate::*;
use trackable::{
    TrackSimple, Trackable, TrackableAtom, TrackableMakeChangeM, TrackableMap,
    TrackableTakeChangeM, TrackableVec, TrackableVecSimple,
};

#[derive(Default)]
pub struct HuskyDevtimeState {
    pub(crate) presentation: TrackableAtom<Presentation>,
    pub(crate) trace_nodes: TrackableVec<TraceNode>,
    pub(crate) figure_canvases: TrackableMap<FigureCanvasKey, FigureCanvasData>,
    pub(crate) figure_controls: TrackableMap<FigureControlKey, FigureControlData>,
    pub(crate) trace_stalks: TrackableMap<TraceStalkKey, TraceStalk>,
    pub(crate) trace_statss: TrackableMap<TraceStatsKey, Option<TraceStats>>,
    root_traces: TrackableVecSimple<TraceId>,
    pub(crate) subtrace_ids_map: TrackableMap<SubtracesKey, Vec<TraceId>>,
}

pub struct DevtimeStateChange {
    pub(crate) restriction: <TrackableAtom<Presentation> as Trackable>::Change,
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

impl Trackable for HuskyDevtimeState {
    type Change = DevtimeStateChange;

    fn take_change(&mut self) -> TrackableTakeChangeM<Self> {
        TrackableTakeChangeM::Ok(DevtimeStateChange {
            restriction: self.presentation.take_change()?,
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

impl HuskyDevtimeState {
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

    pub(crate) fn activate_trace(&mut self, trace_id: TraceId) -> HuskyDevtimeTakeChangeM<()> {
        let trace_data = &self.trace_nodes[trace_id.raw()].trace_data();
        self.presentation
            .update(|presentation| presentation.activate_trace(trace_data));
        HuskyDevtimeTakeChangeM::Ok(())
    }

    pub(crate) fn clear(&mut self) -> HuskyDevtimeUpdateM<()> {
        self.presentation.update(|restriction| restriction.clear());
        self.trace_nodes = Default::default();
        self.figure_canvases = Default::default();
        self.figure_controls = Default::default();
        self.trace_stalks = Default::default();
        self.trace_statss = Default::default();
        self.root_traces = Default::default();
        self.subtrace_ids_map = Default::default();
        HuskyDevtimeUpdateM::Ok(())
    }
}
