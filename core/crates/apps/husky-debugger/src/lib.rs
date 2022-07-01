mod config;
mod error;
pub mod flags;
mod gui;
mod internal;
mod mode;
mod notif;

pub use error::{DebuggerError, DebuggerResult};
use husky_file::FilePtr;
use husky_trace_time::HuskyTraceTime;
pub use mode::Mode;

use avec::Avec;
use config::DebuggerConfig;
use futures::executor::ThreadPool;
use gui::handle_query;
use husky_compile_time::HuskyCompileTime;
use husky_trace_protocol::*;
use internal::HuskyDebuggerInternal;
use json_result::JsonResult;
use notif::handle_notif;
use print_utils::*;
use std::{collections::HashMap, convert::Infallible, net::ToSocketAddrs, sync::Arc};
use std::{sync::Mutex, time::Instant};
use test_utils::TestResult;
use warp::Filter;

pub struct HuskyDebugger {
    internal: Mutex<HuskyDebuggerInternal>,
    threadpool: ThreadPool,
}

impl HuskyDebugger {
    pub fn new(init_compile_time: impl FnOnce(&mut HuskyCompileTime)) -> Self {
        let config = DebuggerConfig::from_env();
        let mut trace_time = HuskyTraceTime::new(init_compile_time, config.verbose);
        if let Some(ref sample_id_str) = config.opt_sample_id {
            let sample_id: SampleId = SampleId(sample_id_str.parse().unwrap());
            trace_time.set_attention(Attention::Specific { sample_id });
        }
        Self {
            internal: Mutex::new(HuskyDebuggerInternal {
                trace_time,
                config,
                next_request_id: 0,
            }),
            threadpool: ThreadPool::new().unwrap(),
        }
    }

    pub async fn serve_on_error(self, addr: impl ToSocketAddrs, sample_id: SampleId) -> TestResult {
        if self.has_root_error(sample_id).await {
            self.serve(addr).await.unwrap();
            TestResult::Failed
        } else {
            TestResult::Success
        }
    }

    async fn has_root_error(&self, sample_id: SampleId) -> bool {
        let mut error_flag = false;
        let internal = &mut self.internal.lock().unwrap();
        internal
            .trace_time
            .set_attention(Attention::Specific { sample_id });
        for root_trace_id in internal.trace_time.root_traces().into_iter() {
            let now = Instant::now();
            let (_, stalk) = internal.trace_time.keyed_trace_stalk(root_trace_id);
            println!(
                "{} milliseconds elapsed for computing stalk of trace {}",
                now.elapsed().as_millis(),
                root_trace_id.0
            );
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

    pub fn change_text(&self) {
        todo!()
    }
}

fn with_debugger(
    debugger: Arc<HuskyDebugger>,
) -> impl Filter<Extract = (Arc<HuskyDebugger>,), Error = Infallible> + Clone {
    warp::any().map(move || debugger.clone())
}
