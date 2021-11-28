mod flychecker;
mod handle;
mod server_sender;
mod server_taskpool;
mod server_vfs;

use std::{sync::Arc, time::Instant};

use crossbeam_channel::{unbounded, Receiver, Sender};
// use ide::{Analysis, AnalysisHost, Cancellable, Change, FileID};
use parking_lot::{Mutex, RwLock};
use rustc_hash::FxHashMap;

use crate::{server_config::ServerConfig, task::Task, taskpool::TaskPool};

use flychecker::FlyChecker;
use handle::Handle;
use server_sender::ServerSender;
use server_taskpool::ServerTaskPool;
use server_vfs::ServerVFS;

pub(crate) struct Server {
    pub(crate) sender: ServerSender,
    pub(crate) config: ServerConfig,
    pub(crate) taskpool: ServerTaskPool,
    pub(crate) vfs: ServerVFS,
    pub(crate) flychecker: FlyChecker,
}

impl Server {
    pub fn new(sender: Sender<lsp_server::Message>, config: ServerConfig) -> Server {
        Server {
            config,
            sender: ServerSender::new(sender),
            taskpool: ServerTaskPool::new(),
            vfs: ServerVFS::new(),
            flychecker: FlyChecker::new(),
        }
    }
}
