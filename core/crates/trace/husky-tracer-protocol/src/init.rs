use super::*;
use crate::*;
use std::{collections::HashMap, rc::Rc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InitData {
    pub focus: Focus,
    pub trace_init_data: TraceInitState,
    pub figures: HashMap<FigureKey, FigureProps>,
    pub figure_controls: HashMap<FigureControlKey, FigureControlProps>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TraceInitState {
    pub trace_nodes: Vec<TraceNodeData>,
    pub opt_active_trace_id: Option<TraceId>,
    pub subtrace_ids_map: HashMap<SubtracesKey, Vec<TraceId>>,
    pub root_trace_ids: Vec<TraceId>,
}
