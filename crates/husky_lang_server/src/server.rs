mod comm;

use std::sync::Arc;

use crossbeam_channel::Sender;

use crate::config::ServerConfig;

use comm::Communicator;

pub(crate) struct Server {
    pub(crate) comm: Communicator,
    pub(crate) config: Arc<ServerConfig>,
    pub(crate) db: husky_lang_db::HuskyLangDatabase,
    pub(crate) threadpool: threadpool::ThreadPool,
}

impl Server {
    pub fn new(sender: Sender<lsp_server::Message>, config: ServerConfig) -> Server {
        Server {
            config: Arc::new(config),
            comm: Communicator::new(sender),
            threadpool: threadpool::ThreadPool::default(),
            db: husky_lang_db::HuskyLangDatabase::default(),
        }
    }
}

pub enum ServerControlSignal {
    Normal,
    Shutdown,
}
