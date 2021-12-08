mod comm;

use std::sync::Arc;

use crossbeam_channel::Sender;
use ide::IdeDatabaseProxy;

use crate::{config::ServerConfig, server_snapshot::ServerSnapshot};

use comm::Communicator;

pub(crate) struct Server {
    pub(crate) comm: Communicator,
    pub(crate) config: Arc<ServerConfig>,
    pub(crate) db: IdeDatabaseProxy,
    pub(crate) threadpool: threadpool::ThreadPool,
}

impl Server {
    pub fn new(sender: Sender<lsp_server::Message>, config: ServerConfig) -> Server {
        let analysis_host = IdeDatabaseProxy::new(config.lru_capacity());
        Server {
            config: Arc::new(config),
            comm: Communicator::new(sender),
            threadpool: threadpool::ThreadPool::default(),
            db: analysis_host,
        }
    }

    pub(crate) fn take_snapshot(&self) -> ServerSnapshot {
        ServerSnapshot {
            db: self.db.snapshot(),
        }
    }
}

pub enum ServerControlSignal {
    Normal,
    Shutdown,
}
