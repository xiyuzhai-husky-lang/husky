use vec_like::VecSet;

use super::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InitData {
    pub restriction: Restriction,
    pub trace_init_data: TraceInitState,
    pub figure_canvases: Vec<(FigureCanvasKey, FigureCanvasData)>,
    pub figure_controls: Vec<(FigureControlKey, FigureControlData)>,
    pub pins: VecSet<TraceId>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TraceInitState {
    pub trace_nodes: Vec<TraceNodeData>,
    pub opt_active_trace_id: Option<TraceId>,
    pub subtrace_ids_map: Vec<(SubtracesKey, Vec<TraceId>)>,
    pub trace_stalks: Vec<(TraceStalkKey, TraceStalkData)>,
    pub root_trace_ids: Vec<TraceId>,
}
