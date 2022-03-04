mod config;
mod error;
pub mod flags;
mod gui;
pub mod mock;
mod notif;
mod state;
#[cfg(test)]
mod tests;

use std::{convert::Infallible, net::ToSocketAddrs, sync::Arc};

use common::{epin, p, HashMap};
use config::DebuggerConfig;
pub use error::{DebuggerError, DebuggerResult};
use husky_lang_compile_time::HuskyLangCompileTime;
use husky_lang_runtime::{HuskyLangRuntime, RuntimeQueryGroup};

use futures::executor::ThreadPool;
use gui::handle_query;
use notif::handle_notif;
use state::DebuggerState;
use std::sync::Mutex;
use trace::{AllocateTrace, FigureProps, Trace, TraceId, TraceStalk};
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
            runtime.lock_input(input_id_str);
        }
        Self {
            runtime: Mutex::new(runtime),
            threadpool: ThreadPool::new().unwrap(),
            state: Default::default(),
            config,
        }
    }

    pub async fn serve(self, addr: impl ToSocketAddrs) -> DebuggerResult<()> {
        let debugger = Arc::new(self);
        let addr = addr.to_socket_addrs().unwrap().next().unwrap();
        println!(
            "{}husky{}: serve on {:?}",
            common::show::CYAN,
            common::show::RESET,
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
        effective_opt_input_id_for_subtraces: Option<usize>,
    ) -> Arc<Vec<Arc<Trace>>> {
        let subtraces = self
            .runtime
            .lock()
            .unwrap()
            .subtraces(trace_id, effective_opt_input_id_for_subtraces);
        self.state.lock().unwrap().set_subtraces(
            trace_id,
            effective_opt_input_id_for_subtraces,
            &subtraces,
        );
        subtraces
    }

    pub fn expansions(&self) -> HashMap<TraceId, bool> {
        self.runtime.lock().unwrap().expansions()
    }

    pub fn showns(&self) -> HashMap<TraceId, bool> {
        self.runtime.lock().unwrap().showns()
    }

    pub async fn figure(&self, id: TraceId) -> Option<FigureProps> {
        self.runtime.lock().unwrap().figure(id)
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

    pub async fn trace(&self, id: TraceId) -> Arc<Trace> {
        self.runtime.lock().unwrap().trace(id)
    }

    pub async fn lock_input(&self, input_str: &str) -> (Option<Option<usize>>, Option<String>) {
        self.runtime.lock().unwrap().lock_input(input_str)
    }

    pub async fn trace_stalk(&self, trace_id: TraceId, input_id: usize) -> Arc<TraceStalk> {
        self.runtime.lock().unwrap().trace_stalk(trace_id, input_id)
    }
}

fn with_debugger(
    debugger: Arc<Debugger>,
) -> impl Filter<Extract = (Arc<Debugger>,), Error = Infallible> + Clone {
    warp::any().map(move || debugger.clone())
}
