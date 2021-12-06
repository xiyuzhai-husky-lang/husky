mod comm;
mod flychecker;
mod fs_keeper;
pub(crate) mod handle;
pub(crate) mod live_docs;
mod taskpool;

use std::{collections::HashMap, sync::Arc, time::Instant};

use crossbeam_channel::{unbounded, Receiver, Sender};
use ide::{AnalysisHost, Cancellable, Change, DatabaseProxy, FileID};
use ide_db::base_db::{CrateId, FileLoader, SourceDatabase};
use lsp_types::{SemanticTokens, Url};
use parking_lot::{Mutex, RwLock};
use rustc_hash::FxHashMap;

use crate::{
    config::ServerConfig, diagnostics, lsp_ext, op_queue::OpnQueue, reload,
    server_snapshot::ServerSnapshot, task::Task, taskpool::TaskPool,
};

use comm::Communicator;
use flychecker::FlyChecker;
use fs_keeper::ServerFileSystemKeeper;
use handle::Handle;
use live_docs::LiveDocs;
use taskpool::ServerTaskPool;

pub(crate) struct Server {
    pub(crate) comm: Communicator,
    pub(crate) config: Arc<ServerConfig>,
    pub(crate) analysis_host: AnalysisHost,
    pub(crate) diagnostics: diagnostics::DiagnosticsTracker,
    pub(crate) taskpool: ServerTaskPool,
    pub(crate) vfs: ServerFileSystemKeeper,
    pub(crate) flychecker: FlyChecker,
    pub(crate) live_docs: LiveDocs,
    pub(crate) prime_caches_queue: OpnQueue<()>,
    pub(crate) semantic_tokens_cache: Arc<Mutex<FxHashMap<Url, SemanticTokens>>>,
    pub(crate) last_reported_status: Option<lsp_ext::ServerStatusParams>,
    pub(crate) shutdown_requested: bool,
}

impl Server {
    pub fn new(sender: Sender<lsp_server::Message>, config: ServerConfig) -> Server {
        let analysis_host = AnalysisHost::new(config.lru_capacity());
        Server {
            config: Arc::new(config),
            comm: Communicator::new(sender),
            taskpool: ServerTaskPool::new(),
            vfs: ServerFileSystemKeeper::new(),
            flychecker: FlyChecker::new(),
            live_docs: LiveDocs::default(),
            analysis_host,
            diagnostics: diagnostics::DiagnosticsTracker::default(),
            prime_caches_queue: OpnQueue::<()>::default(),
            semantic_tokens_cache: Arc::new(Default::default()),
            last_reported_status: None,
            shutdown_requested: false,
        }
    }

    pub(crate) fn process_changes(&mut self) -> bool {
        let _p = profile::span("Server::process_changes");

        if let Some(change) = self.vfs.internal.write().pipe_changes() {
            self.analysis_host.apply_change(change);
            return true;
        } else {
            return false;
        }
    }

    pub(crate) fn is_quiescent(&self) -> bool {
        !(self.vfs.progress_config_version < self.vfs.config_version
            || self.vfs.progress_n_done < self.vfs.progress_n_total)
    }

    pub(crate) fn take_snapshot(&self) -> ServerSnapshot {
        ServerSnapshot {
            config: Arc::clone(&self.config),
            analysis: self.analysis_host.analysis(),
            vfs: Arc::clone(&self.vfs.internal),
            check_fixes: Arc::clone(&self.diagnostics.check_fixes),
            live_docs: self.live_docs.clone(),
            semantic_tokens_cache: Arc::clone(&self.semantic_tokens_cache),
        }
    }
}
