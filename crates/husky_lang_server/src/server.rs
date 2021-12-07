mod comm;
pub(crate) mod handle;
pub(crate) mod live_docs;
mod taskpool;

use std::sync::Arc;

use crossbeam_channel::Sender;
use ide::IdeDatabaseProxy;
use parking_lot::RwLock;

use crate::{config::ServerConfig, lsp_ext, server_snapshot::ServerSnapshot};

use comm::Communicator;
use live_docs::LiveDocs;
use taskpool::ServerTaskPool;

pub(crate) struct Server {
    pub(crate) comm: Communicator,
    pub(crate) config: Arc<ServerConfig>,
    pub(crate) ide_db_proxy: IdeDatabaseProxy,
    pub(crate) taskpool: ServerTaskPool,
    pub(crate) vfs: Arc<RwLock<vfs::Vfs>>,
    pub(crate) live_docs: LiveDocs,
    pub(crate) last_reported_status: Option<lsp_ext::ServerStatusParams>,
}

impl Server {
    pub fn new(sender: Sender<lsp_server::Message>, config: ServerConfig) -> Server {
        let analysis_host = IdeDatabaseProxy::new(config.lru_capacity());
        Server {
            config: Arc::new(config),
            comm: Communicator::new(sender),
            taskpool: ServerTaskPool::new(),
            vfs: Arc::new(RwLock::new(vfs::Vfs::default())),
            live_docs: LiveDocs::default(),
            ide_db_proxy: analysis_host,
            last_reported_status: None,
        }
    }

    pub(crate) fn process_changes(&mut self) -> bool {
        let _p = profile::span("Server::process_changes");
        let file_changes = self.vfs.write().drain_changes();
        let has_changed = file_changes.len() > 0;
        self.ide_db_proxy.apply_change(file_changes);
        has_changed
    }

    pub(crate) fn take_snapshot(&self) -> ServerSnapshot {
        ServerSnapshot {
            config: Arc::clone(&self.config),
            analysis: self.ide_db_proxy.analysis(),
            vfs: Arc::clone(&self.vfs),
        }
    }
}
