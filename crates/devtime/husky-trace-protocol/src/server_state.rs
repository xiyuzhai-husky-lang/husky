use std::{collections::HashMap, marker::PhantomData};

use crate::*;
use trackable::*;

pub trait AsTraceNode: TrackClone<CloneOutput = TraceNodeData> {
    type Sketch: AsTraceSketch<Node = Self>;

    fn trace_data(&self) -> &TraceData;

    fn trace_id(&self) -> TraceId {
        self.trace_data().id
    }
}

pub trait AsTraceSketch: Eq + std::hash::Hash + Sized {
    type Node: AsTraceNode;
    fn new(node: &Self::Node) -> Option<Self>;
}

pub struct ServerTraceState<TraceNode: AsTraceNode> {
    pub(crate) presentation: TrackableAtom<Presentation>,
    pub trace_nodes: TrackableVec<TraceNode>,
    pub generic_figure_canvases: TrackableMap<GenericFigureCanvasKey, GenericFigureCanvasData>,
    pub specific_figure_canvases: TrackableMap<SpecificFigureCanvasKey, SpecificFigureCanvasData>,
    pub figure_controls: TrackableMap<FigureControlKey, FigureControlData>,
    pub trace_stalks: TrackableMap<TraceStalkKey, TraceStalk>,
    pub trace_statss: TrackableMap<TraceStatsKey, Option<TraceStats>>,
    root_traces: TrackableVecSimple<TraceId>,
    pub subtrace_ids_map: TrackableMap<SubtracesKey, Vec<TraceId>>,
}

impl<TraceNode: AsTraceNode> Default for ServerTraceState<TraceNode> {
    fn default() -> Self {
        Self {
            presentation: Default::default(),
            trace_nodes: Default::default(),
            generic_figure_canvases: Default::default(),
            specific_figure_canvases: Default::default(),
            figure_controls: Default::default(),
            trace_stalks: Default::default(),
            trace_statss: Default::default(),
            root_traces: Default::default(),
            subtrace_ids_map: Default::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerTraceStateChange {
    pub trace_nodes: TrackableVecChange<TraceNodeData>,
    pub generic_figure_canvases:
        <TrackableMap<GenericFigureCanvasKey, GenericFigureCanvasData> as Trackable>::Change,
    pub specific_figure_canvases:
        <TrackableMap<SpecificFigureCanvasKey, SpecificFigureCanvasData> as Trackable>::Change,
    pub figure_controls: <TrackableMap<FigureControlKey, FigureControlData> as Trackable>::Change,
    pub trace_stalks: <TrackableMap<TraceStalkKey, TraceStalk> as Trackable>::Change,
    pub trace_statss: <TrackableMap<TraceStatsKey, Option<TraceStats>> as Trackable>::Change,
    pub root_traces: <TrackableVecSimple<TraceId> as Trackable>::Change,
    pub subtrace_ids_map: <TrackableMap<SubtracesKey, Vec<TraceId>> as Trackable>::Change,
    pub presentation: <TrackableAtom<Presentation> as Trackable>::Change,
}

// implementation details
impl<TraceNode: AsTraceNode> Trackable for ServerTraceState<TraceNode> {
    type Change = ServerTraceStateChange;

    fn take_change(&mut self) -> Self::Change {
        ServerTraceStateChange {
            presentation: self.presentation.take_change(),
            trace_nodes: self.trace_nodes.take_change(),
            specific_figure_canvases: self.specific_figure_canvases.take_change(),
            generic_figure_canvases: self.generic_figure_canvases.take_change(),
            figure_controls: self.figure_controls.take_change(),
            trace_stalks: self.trace_stalks.take_change(),
            trace_statss: self.trace_statss.take_change(),
            root_traces: self.root_traces.take_change(),
            subtrace_ids_map: self.subtrace_ids_map.take_change(),
        }
    }
}

impl<TraceNode: AsTraceNode> ServerTraceState<TraceNode> {
    pub fn presentation(&self) -> &Presentation {
        &self.presentation
    }
    pub fn set_presentation(&mut self, presentation: Presentation) {
        self.presentation.set(presentation)
    }

    pub fn update_presentation(&mut self, f: impl FnOnce(&mut Presentation)) {
        self.presentation.update(f)
    }

    pub fn root_traces(&self) -> &[TrackSimple<TraceId>] {
        &self.root_traces
    }

    pub fn set_root_traces(&mut self, root_traces: Vec<TraceId>) {
        self.root_traces
            .set(root_traces.into_iter().map(|id| id.into()).collect())
    }

    pub fn activate_trace(&mut self, trace_id: TraceId) {
        let trace_data = &self.trace_nodes[trace_id.raw()].trace_data();
        self.presentation
            .update(|presentation| presentation.activate_trace(trace_data))
    }

    pub fn clear_pop(&mut self) -> ServerTraceOldState<TraceNode> {
        let presentation = self.presentation.clear_pop();
        let trace_nodes = self.trace_nodes.clear_pop();
        self.specific_figure_canvases = Default::default();
        self.generic_figure_canvases = Default::default();
        self.figure_controls = Default::default();
        self.trace_stalks = Default::default();
        self.trace_statss = Default::default();
        self.root_traces = Default::default();
        self.subtrace_ids_map = Default::default();
        ServerTraceOldState::new(presentation, trace_nodes)
    }
}

#[must_use]
pub struct ServerTraceOldState<T: AsTraceNode> {
    presentation: Presentation,
    trace_nodes: Vec<T>,
    trace_id_map: Vec<TraceIdMatch>,
    trace_sketches: HashMap<T::Sketch, TraceId>,
    fixed: bool,
}

impl<TraceNode: AsTraceNode> ServerTraceOldState<TraceNode> {
    pub fn new(presentation: Presentation, trace_nodes: Vec<TraceNode>) -> Self {
        let trace_sketches = trace_nodes
            .iter()
            .filter_map(|trace_node| {
                TraceNode::Sketch::new(trace_node).map(|sketch| (sketch, trace_node.trace_id()))
            })
            .collect();
        Self {
            presentation,
            trace_nodes,
            trace_id_map: vec![],
            trace_sketches,
            fixed: false,
        }
    }

    pub fn try_match_node(&mut self, new_node: &TraceNode) -> Option<&TraceNode> {
        let new_id = new_node.trace_id();
        assert!(self.try_match_id(new_id).is_none());
        let sketch = TraceNode::Sketch::new(new_node)?;
        let old_id = self.trace_sketches.get(&sketch)?;
        let old_node = &self.trace_nodes[old_id.raw()];
        self.trace_id_map.push(TraceIdMatch {
            old_id: *old_id,
            new_id,
        });
        Some(old_node)
    }

    pub fn try_match_id(&self, new: TraceId) -> Option<TraceId> {
        self.trace_id_map
            .iter()
            .find(|m| m.new_id == new)
            .map(|m| m.old_id)
    }

    pub fn fix(&mut self) {
        assert!(!self.fixed);
        self.fixed = true;
    }

    pub fn mimic_presentation(&self, trace_nodes: &[TraceNode]) -> Presentation {
        assert!(self.fixed);
        self.presentation.mimic(&|id| {
            self.trace_id_map
                .iter()
                .find(|m| m.old_id == id)
                .map(|m| trace_nodes[m.new_id.raw()].trace_data())
        })
    }
}

pub struct TraceIdMatch {
    pub old_id: TraceId,
    pub new_id: TraceId,
}
