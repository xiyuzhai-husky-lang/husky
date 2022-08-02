mod config;
mod error;
mod flags;
mod gui;
mod internal;
mod mode;
mod notif;

pub use config::HuskyDebuggerConfig;
pub use error::{DebuggerError, DebuggerResult};
pub use flags::HuskyDebuggerFlags;
pub use mode::Mode;

use avec::Avec;
use futures::executor::ThreadPool;
use gui::handle_query;
use husky_compile_time::HuskyCompileTime;
use husky_file::FilePtr;
use husky_print_utils::*;
use husky_root_static_defn::__StaticLinkageKey;
use husky_test_utils::TestResult;
use husky_trace_protocol::*;
use husky_trace_time::HuskyTraceTime;
use internal::HuskyDebuggerInternal;
use json_result::JsonResult;
use notif::handle_notif;
use std::{
    collections::HashMap,
    convert::Infallible,
    net::ToSocketAddrs,
    path::{Path, PathBuf},
    sync::Arc,
};
use std::{sync::Mutex, time::Instant};
use vm::__Linkage;
use warp::Filter;

pub type GetLinkagesFromCDylib =
    unsafe extern "C" fn() -> &'static [(__StaticLinkageKey, __Linkage)];

pub struct HuskyDebugger {
    pub(crate) config: HuskyDebuggerConfig,
    internal: Mutex<HuskyDebuggerInternal>,
    threadpool: ThreadPool,
}

impl HuskyDebugger {
    pub fn new_from_flags() -> Self {
        let mut config = HuskyDebuggerConfig::from_env();
        HuskyDebugger::new(config, &[])
    }
    pub fn new(config: HuskyDebuggerConfig, linkages: &[(__StaticLinkageKey, __Linkage)]) -> Self {
        let package_dir: &Path = &config.package_dir;
        let mut trace_time = HuskyTraceTime::new(
            |compile_time| {
                compile_time.load_package(package_dir);
                compile_time.load_linkages(linkages)
            },
            config.eval_time(),
        );
        if let Some(specific_sample_id) = config.opt_sample_id {
            trace_time.set_restriction(Restriction::new_specific(specific_sample_id));
        }
        Self {
            internal: Mutex::new(HuskyDebuggerInternal {
                trace_time,
                next_request_id: 0,
            }),
            config,
            threadpool: ThreadPool::new().unwrap(),
        }
    }

    pub async fn serve_on_error(self, addr: impl ToSocketAddrs, sample_id: SampleId) -> TestResult {
        if self.has_root_error(sample_id).await {
            self.serve(addr).await.unwrap();
            TestResult::Failure
        } else {
            TestResult::Success
        }
    }

    async fn has_root_error(&self, specific_sample_id: SampleId) -> bool {
        let mut error_flag = false;
        let internal = &mut self.internal.lock().unwrap();
        internal
            .trace_time
            .set_restriction(Restriction::new_specific(specific_sample_id));
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
            husky_print_utils::CYAN,
            husky_print_utils::RESET,
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
