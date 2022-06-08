use std::collections::HashMap;

use super::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InitState {
    pub active_trace_id: Option<TraceId>,
    pub focus: Focus,
    pub traces: Vec<TraceProps>,
    pub subtraces_list: HashMap<(TraceId, Option<usize>), Vec<TraceId>>,
    pub root_traces: Vec<TraceId>,
    pub expansions: HashMap<TraceId, bool>,
    pub showns: HashMap<TraceId, bool>,
    pub figures: HashMap<String, FigureProps>,
    pub figure_controls: HashMap<String, FigureControlProps>,
}
