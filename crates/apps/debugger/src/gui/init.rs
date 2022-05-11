use json_map::JsonListMap;
use runtime_db::FigureControlProps;
use trace::TraceFactory;
use wild_utils::ref_to_mut_ref;

use super::*;
use crate::*;

// fn init_gui(debugger: &Debugger, sender: UnboundedSender<Result<Message, warp::Error>>) {
//     let root_traces = debugger.root_traces();
//     let expansions = debugger.expansions();
//     let showns = debugger.showns();
//     let state = debugger.state.lock().unwrap();
//     let mut runtime = debugger.runtime.lock().unwrap();
//     let focus = runtime.focus();
//     let mut figures = HashMap::default();
//     let mut figure_controls = HashMap::default();
//     let active_trace_id = state.active_trace_id;
//     if let Some(active_trace_id) = active_trace_id {
//         let active_trace = runtime.trace(active_trace_id);
//         figures.insert(
//             focus.figure_key(active_trace_id),
//             runtime.figure(active_trace_id, focus),
//         );
//         figure_controls.insert(
//             focus.figure_control_key(&active_trace),
//             unsafe { ref_to_mut_ref(&runtime) }.figure_control(&active_trace, focus),
//         );
//     }
//     let traces = runtime.traces();
//     msg_once!("init figures");
//     let response = ResponseVariant::Init {
//         init_state: InitState {
//             active_trace_id,
//             focus,
//             traces,
//             subtraces_list: &state.subtraces_map,
//             root_traces: &root_traces,
//             expansions: &expansions,
//             showns: &showns,
//             figures,
//             figure_controls,
//         },
//     };
//     match sender.send(Ok(Message::text(serde_json::to_string(&response).unwrap()))) {
//         Ok(_) => {
//             println!("init message sent")
//         }
//         Err(_) => todo!(),
//     };
// }

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
