use json_map::JsonMap;
use trace::TraceFactory;

use super::*;
use crate::*;

pub fn init_gui(debugger: &Debugger, sender: UnboundedSender<Result<Message, warp::Error>>) {
    let root_traces = debugger.root_traces();
    let expansions = debugger.expansions();
    let showns = debugger.showns();
    let state = debugger.state.lock().unwrap();
    let runtime = debugger.runtime.lock().unwrap();
    let traces = runtime.traces();
    let response = Response::Init {
        init_state: InitState {
            active_trace_id: state.active_trace_id,
            opt_input_id: runtime.opt_input_id(),
            traces,
            subtraces_list: &state.subtraces_map,
            root_traces: &root_traces,
            expansions: &expansions,
            showns: &showns,
        },
    };
    match sender.send(Ok(Message::text(serde_json::to_string(&response).unwrap()))) {
        Ok(_) => {
            println!("init message sent")
        }
        Err(_) => todo!(),
    };
}

#[derive(Debug, Serialize, Clone)]
pub struct InitState<'a> {
    pub active_trace_id: Option<TraceId>,
    pub opt_input_id: Option<usize>,
    pub traces: &'a TraceFactory,
    pub subtraces_list: &'a JsonMap<(TraceId, Option<usize>), Vec<TraceId>>,
    pub root_traces: &'a [TraceId],
    pub expansions: &'a HashMap<TraceId, bool>,
    pub showns: &'a HashMap<TraceId, bool>,
}
