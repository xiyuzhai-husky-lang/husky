use json_map::JsonListMap;
use runtime_db::FigureControlProps;
use trace::TraceFactory;

use super::*;
use crate::*;

#[derive(Debug, Serialize, Clone)]
pub struct InitState<'a> {
    pub active_trace_id: Option<TraceId>,
    pub focus: &'a Focus,
    pub traces: &'a TraceFactory<'static>,
    pub subtraces_list: &'a JsonListMap<(TraceId, Option<usize>), Vec<TraceId>>,
    pub root_traces: Arc<Vec<TraceId>>,
    pub expansions: &'a HashMap<TraceId, bool>,
    pub showns: &'a HashMap<TraceId, bool>,
    pub figures: HashMap<String, FigureProps>,
    pub figure_controls: HashMap<String, FigureControlProps>,
}
