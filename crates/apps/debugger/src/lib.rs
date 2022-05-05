mod config;
mod error;
pub mod flags;
mod gui;
pub mod mock;
mod mode;
mod notif;
mod state;
#[cfg(test)]
mod tests;

use avec::Avec;
pub use error::{DebuggerError, DebuggerResult};
use focus::Focus;
use json_result::JsonResult;
pub use mode::Mode;

use compile_time_db::HuskyLangCompileTime;
use config::DebuggerConfig;
use futures::executor::{block_on, ThreadPool};
use gui::handle_query;
use notif::handle_notif;
use print_utils::*;
use runtime_db::{HuskyLangRuntime, RuntimeQueryGroup};
use state::DebuggerState;
use std::sync::Mutex;
use std::{collections::HashMap, convert::Infallible, net::ToSocketAddrs, path::Path, sync::Arc};
use trace::{CreateTrace, FigureProps, Trace, TraceId, TraceStalk, TraceTokenKind};
use warp::Filter;

pub struct Debugger {
    runtime: Mutex<HuskyLangRuntime>,
    threadpool: ThreadPool,
    state: Mutex<DebuggerState>,
    config: DebuggerConfig,
}

impl Debugger {
    pub fn new(init_compile_time: impl FnOnce(&mut HuskyLangCompileTime)) -> Self {
        let config = DebuggerConfig::from_env();
        let mut runtime = HuskyLangRuntime::new(init_compile_time);
        if let Some(ref input_id_str) = config.opt_input_id {
            match runtime.lock_input(input_id_str) {
                (_, Some(msg)) => panic!("{}", msg),
                (Some(Some(input_id)), None) => {
                    for trace in runtime.root_traces().iter() {
                        let stalk = runtime.trace_stalk(*trace, input_id);
                    }
                }
                _ => (),
            }
        }
        Self {
            runtime: Mutex::new(runtime),
            threadpool: ThreadPool::new().unwrap(),
            state: Default::default(),
            config,
        }
    }

    pub async fn serve_on_error(self, addr: impl ToSocketAddrs, input_id: usize) -> bool {
        let mut error_flag = false;
        for trace in self.root_traces().iter() {
            let stalk = self.trace_stalk(*trace, input_id).await;
            for token in &stalk.extra_tokens {
                match token.kind {
                    TraceTokenKind::Error => {
                        error_flag = true;
                        break;
                    }
                    _ => (),
                }
            }
        }
        if error_flag {
            self.serve(addr).await.unwrap()
        }
        error_flag
    }

    pub async fn serve(self, addr: impl ToSocketAddrs) -> DebuggerResult<()> {
        let debugger = Arc::new(self);
        let addr = addr.to_socket_addrs().unwrap().next().unwrap();
        println!(
            "{}husky{}: serve on {:?}",
            print_utils::CYAN,
            print_utils::RESET,
            addr
        );
        let notif = warp::path!("notif")
            .and(warp::ws())
            .and(with_debugger(debugger.clone()))
            .and_then(handle_notif);
        let query = warp::path!("query")
            .and(warp::ws())
            .and(with_debugger(debugger.clone()))
            .and_then(handle_query);
        let routes = notif.or(query);
        warp::serve(routes).run(addr).await;
        Ok(())
    }

    pub fn change_text(&self) {}

    pub fn root_traces(&self) -> Arc<Vec<TraceId>> {
        self.runtime.lock().unwrap().root_traces()
    }

    pub async fn subtraces(
        &self,
        trace_id: TraceId,
        effective_opt_input_id: Option<usize>,
    ) -> Avec<Trace<'static>> {
        let runtime = self.runtime.lock().unwrap();
        let trace = runtime.trace(trace_id);
        let subtraces = runtime.subtraces(trace_id, effective_opt_input_id);
        self.state
            .lock()
            .unwrap()
            .set_subtraces(&trace, effective_opt_input_id, &subtraces);
        subtraces
    }

    pub fn expansions(&self) -> HashMap<TraceId, bool> {
        self.runtime.lock().unwrap().expansions()
    }

    pub fn showns(&self) -> HashMap<TraceId, bool> {
        self.runtime.lock().unwrap().showns()
    }

    pub async fn figure(&self, id: TraceId, focus: &Focus) -> FigureProps {
        self.runtime.lock().unwrap().figure(id, focus)
    }

    pub async fn activate(&self, id: TraceId) {
        self.state.lock().unwrap().active_trace_id = Some(id);
    }

    pub async fn toggle_expansion(&self, id: TraceId) {
        self.runtime.lock().unwrap().toggle_expansion(id)
    }

    pub async fn toggle_show(&self, id: TraceId) {
        self.runtime.lock().unwrap().toggle_show(id)
    }

    pub async fn trace(&self, id: TraceId) -> Arc<Trace<'static>> {
        self.runtime.lock().unwrap().trace(id)
    }

    pub fn decode_focus(&self, command: &str) -> JsonResult<Focus> {
        self.runtime.lock().unwrap().decode_focus(command)
    }

    pub async fn lock_input(&self, input_str: &str) -> (Option<Option<usize>>, Option<String>) {
        self.runtime.lock().unwrap().lock_input(input_str)
    }

    pub async fn trace_stalk(
        &self,
        trace_id: TraceId,
        input_id: usize,
    ) -> Arc<TraceStalk<'static>> {
        self.runtime.lock().unwrap().trace_stalk(trace_id, input_id)
    }
}

fn with_debugger(
    debugger: Arc<Debugger>,
) -> impl Filter<Extract = (Arc<Debugger>,), Error = Infallible> + Clone {
    warp::any().map(move || debugger.clone())
}
