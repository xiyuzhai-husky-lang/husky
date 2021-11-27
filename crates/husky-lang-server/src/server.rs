use std::{sync::Arc, time::Instant};

use crossbeam_channel::{unbounded, Receiver, Sender};
use ide::{Analysis, AnalysisHost, Cancellable, Change, FileID};
use parking_lot::{Mutex, RwLock};
use rustc_hash::FxHashMap;

use crate::{server_cfg::ServerCfg, task::Task, taskpool::TaskPool};

// Enforces drop order
pub(crate) struct Handle<H, C> {
    pub(crate) handle: H,
    pub(crate) receiver: C,
}

pub(crate) struct Server {
    pub(crate) taskpool: Handle<TaskPool<Task>, Receiver<Task>>,
    pub(crate) vfs: VFS,
    pub(crate) flychecker: FlyChecker,
}

pub(crate) struct VFS {
    pub(crate) loader: Handle<Box<dyn vfs::loader::Handle>, Receiver<vfs::loader::Message>>,
    pub(crate) vfs: Arc<RwLock<(vfs::Vfs, FxHashMap<FileID, LineEndings>)>>,
    pub(crate) vfs_config_version: u32,
    pub(crate) vfs_progress_config_version: u32,
    pub(crate) vfs_progress_n_total: usize,
    pub(crate) vfs_progress_n_done: usize,
}

pub(crate) struct FlyChecker {
    pub(crate) receiver: Receiver<flycheck::Message>,
}

impl Server {
    pub fn new() -> Server {
        todo!()
    }
}
