use crossbeam_channel::{unbounded, Receiver, Sender};

use crate::server::handle::Handle;

pub(crate) struct ServerVFS {
    pub(crate) loader: Handle<Box<dyn vfs::loader::Handle>, Receiver<vfs::loader::Message>>,
    // pub(crate) vfs: Arc<RwLock<(vfs::Vfs, FxHashMap<FileID, LineEndings>)>>,
    // pub(crate) vfs_config_version: u32,
    // pub(crate) vfs_progress_config_version: u32,
    // pub(crate) vfs_progress_n_total: usize,
    // pub(crate) vfs_progress_n_done: usize,
}

impl ServerVFS {
    pub(super) fn new() -> ServerVFS {
        let (sender, receiver) = unbounded::<vfs::loader::Message>();
        let handle: vfs_notify::NotifyHandle =
            vfs::loader::Handle::spawn(Box::new(move |msg| sender.send(msg).unwrap()));
        let handle = Box::new(handle) as Box<dyn vfs::loader::Handle>;
        ServerVFS {
            loader: Handle { handle, receiver },
        }
    }
}
