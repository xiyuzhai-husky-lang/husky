mod error;
mod notif;
mod query;
mod response;
mod tests;

use std::{convert::Infallible, net::ToSocketAddrs, sync::Arc};

pub use error::{DebuggerError, DebuggerResult};

use husky_lang_runtime::HuskyLangRuntime;

use futures::executor::ThreadPool;
use notif::handle_notif;
use query::handle_query;
use warp::{http::StatusCode, Filter};

pub struct Debugger {
    pub(crate) runtime: HuskyLangRuntime,
    pub(crate) threadpool: ThreadPool,
}

impl Debugger {
    pub fn new() -> Self {
        Self {
            runtime: Default::default(),
            threadpool: ThreadPool::new().unwrap(),
        }
    }

    pub async fn serve(self, addr: impl ToSocketAddrs) -> DebuggerResult<()> {
        let debugger = Arc::new(self);
        let addr = addr.to_socket_addrs().unwrap().next().unwrap();
        println!(
            "{}husky{}:  serve on {:?}",
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
}

fn with_debugger(
    debugger: Arc<Debugger>,
) -> impl Filter<Extract = (Arc<Debugger>,), Error = Infallible> + Clone {
    warp::any().map(move || debugger.clone())
}
