use std::{sync::Arc, time::Instant};

use crossbeam_channel::{unbounded, Receiver, Sender};
use parking_lot::{Mutex, RwLock};
use rustc_hash::FxHashMap;

use ide::{AnalysisHost, Cancellable, Change, DatabaseProxy, FileID};

use crate::{
    fs::ServerFileSystem,
    line_index::{LineCollection, LineEndingType},
    server::handle::Handle,
    source_root_config::SourceRootConfig,
};

pub(crate) struct ServerFileSystemKeeper {
    pub(crate) loader: Handle<Box<dyn vfs::loader::Handle>, Receiver<vfs::loader::Message>>,
    pub(crate) internal: Arc<RwLock<ServerFileSystem>>,
    pub(crate) config_version: u32,
    pub(crate) progress_config_version: u32,
    pub(crate) progress_n_total: usize,
    pub(crate) progress_n_done: usize,
}

impl ServerFileSystemKeeper {
    pub(super) fn new() -> ServerFileSystemKeeper {
        let (sender, receiver) = unbounded::<vfs::loader::Message>();
        let handle: vfs_notify::NotifyHandle =
            vfs::loader::Handle::spawn(Box::new(move |msg| sender.send(msg).unwrap()));
        let handle = Box::new(handle) as Box<dyn vfs::loader::Handle>;
        ServerFileSystemKeeper {
            loader: Handle { handle, receiver },
            internal: Arc::new(RwLock::new(ServerFileSystem::default())),
            config_version: 0,
            progress_config_version: 0,
            progress_n_total: 0,
            progress_n_done: 0,
        }
    }
}
