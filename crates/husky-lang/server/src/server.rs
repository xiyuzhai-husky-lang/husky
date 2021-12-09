mod comm;

use crossbeam_channel::Sender;

use comm::Communicator;

pub(crate) struct Server {
    pub(crate) comm: Communicator,
    pub(crate) db: husky_lang_db::HuskyLangDatabase,
    pub(crate) threadpool: threadpool::ThreadPool,
}

impl Server {
    pub fn new(sender: Sender<lsp_server::Message>) -> Server {
        Server {
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
