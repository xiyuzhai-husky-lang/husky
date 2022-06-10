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
    pub active_trace_id: Option<TraceId>,
    pub traces: Vec<TraceProps>,
    pub subtraces_map: HashMap<SubtracesKey, Vec<TraceId>>,
    pub root_traces: Vec<TraceId>,
    pub expansions: HashMap<TraceId, bool>,
    pub showns: HashMap<TraceId, bool>,
}
