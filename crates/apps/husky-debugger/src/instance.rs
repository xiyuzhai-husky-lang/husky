use husky_compiler::CompilerInstance;
use relative_path::RelativePathBuf;

use super::*;

pub(crate) struct HuskyDebuggerInstance {
    pub(crate) internal: Mutex<HuskyDebuggerInternal>,
    pub(crate) threadpool: ThreadPool,
}

impl HuskyDebuggerInstance {
    pub fn new(config: HuskyDebuggerConfig, linkages: &[(__StaticLinkageKey, __Linkage)]) -> Self {
        let package_dir: &Path = &config.package_dir;
        let mut debugtime = Debugtime::new(
            |runtime| {
                todo!()
                // runtime.load_package(package_dir);
                // runtime.load_linkages(linkages)
            },
            config.runtime(),
        );
        if let Some(specific_sample_id) = config.opt_sample_id {
            todo!()
            // debugtime
            //     .set_restriction(Restriction::new_specific(specific_sample_id))
            //     .expect("todo");
        }
        Self {
            internal: Mutex::new(HuskyDebuggerInternal {
                debugtime,
                next_request_id: 0,
                config,
            }),
            threadpool: ThreadPool::new().unwrap(),
        }
    }

    pub(crate) fn config(&self) -> HuskyDebuggerConfig {
        self.internal.lock().unwrap().config.clone()
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
        todo!()
        // let mut error_flag = false;
        // let internal = &mut self.internal.lock().unwrap();
        // internal
        //     .debugtime
        //     .set_restriction(Restriction::new_specific(specific_sample_id))
        //     .expect("todo");
        // for root_trace_id in internal.debugtime.root_traces().into_iter() {
        //     let now = Instant::now();
        //     let (_, stalk) = internal.debugtime.keyed_trace_stalk(root_trace_id);
        //     println!(
        //         "{} milliseconds elapsed for computing stalk of trace {}",
        //         now.elapsed().as_millis(),
        //         root_trace_id.0
        //     );
        //     for token in &stalk.extra_tokens {
        //         match token.kind {
        //             TraceTokenKind::Error => {
        //                 error_flag = true;
        //                 break;
        //             }
        //             _ => (),
        //         }
        //     }
        // }
        // error_flag
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
    debugger: Arc<HuskyDebuggerInstance>,
) -> impl Filter<Extract = (Arc<HuskyDebuggerInstance>,), Error = Infallible> + Clone {
    warp::any().map(move || debugger.clone())
}
