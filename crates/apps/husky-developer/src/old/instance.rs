use super::*;

pub(crate) struct HuskyDeveloperInstance {
    pub(crate) internal: Mutex<HuskyDeveloperInternal>,
    pub(crate) threadpool: ThreadPool,
}

impl HuskyDeveloperInstance {
    pub fn new(config: HuskyDeveloperConfig) -> Self {
        let _package_dir: &Path = &config.package_dir;
        let devtime = Devtime::new(config.runtime());
        if let Some(_specific_sample_id) = config.opt_sample_id {
            todo!()
            // devtime
            //     .set_presentation(Restriction::new_specific(specific_sample_id))
            //     .expect("todo");
        }
        Self {
            internal: Mutex::new(HuskyDeveloperInternal {
                devtime,
                next_request_id: 0,
                config,
            }),
            threadpool: ThreadPool::new().unwrap(),
        }
    }

    pub(crate) fn config(&self) -> HuskyDeveloperConfig {
        self.internal.lock().unwrap().config.clone()
    }

    pub async fn serve_on_error(self, addr: impl ToSocketAddrs, sample_id: SampleId) -> TestResult {
        if self.has_root_error(sample_id).await {
            self.serve(addr).await.unwrap();
            TestResult::Err
        } else {
            TestResult::Ok
        }
    }

    async fn has_root_error(&self, _specific_sample_id: SampleId) -> bool {
        todo!()
        // let mut error_flag = false;
        // let internal = &mut self.internal.lock().unwrap();
        // internal
        //     .devtime
        //     .set_presentation(Restriction::new_specific(specific_sample_id))
        //     .expect("todo");
        // for root_trace_id in internal.devtime.root_traces().into_iter() {
        //     let now = Instant::now();
        //     let (_, stalk) = internal.devtime.keyed_trace_stalk(root_trace_id);
        //     println!(
        //         "{} milliseconds elapsed for computing stalk of trace {}",
        //         now.elapsed().as_millis(),
        //         root_trace_id.raw()
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

    pub async fn serve(self, addr: impl ToSocketAddrs) -> DeveloperResult<()> {
        let dev = Arc::new(self);
        let addr = addr.to_socket_addrs().unwrap().next().unwrap();
        println!(
            "{}husky{}: serve on {:?}",
            husky_print_utils::CYAN,
            husky_print_utils::RESET,
            addr
        );
        let notif = warp::path!("notif")
            .and(warp::ws())
            .and(with_dev(dev.clone()))
            .and_then(handle_notif);
        let query = warp::path!("query")
            .and(warp::ws())
            .and(with_dev(dev.clone()))
            .and_then(handle_query);
        let routes = notif.or(query);
        warp::serve(routes).run(addr).await;
        Ok(())
    }

    pub fn change_text(&self) {
        todo!()
    }
}

fn with_dev(
    dev: Arc<HuskyDeveloperInstance>,
) -> impl Filter<Extract = (Arc<HuskyDeveloperInstance>,), Error = Infallible> + Clone {
    warp::any().map(move || dev.clone())
}
