pub(crate) mod client_comm;
pub(crate) mod event_loop_comm;

use crossbeam_channel::Sender;

use client_comm::ClientCommunicator;
use event_loop_comm::EventLoopCommunicator;
use husky_comptime::HuskyComptime;
use husky_root_static_defn::__resolve_root_defn;
use threadpool::ThreadPool;

pub(crate) struct Server {
    pub(crate) client_comm: ClientCommunicator,
    pub(crate) event_loop_comm: EventLoopCommunicator,
    pub(crate) db: HuskyComptime,
    pub(crate) threadpool: ThreadPool,
}

impl Server {
    pub fn new(sender: Sender<lsp_server::Message>) -> Server {
        Server {
            client_comm: ClientCommunicator::new(sender),
            threadpool: ThreadPool::default(),
            event_loop_comm: EventLoopCommunicator::default(),
            db: HuskyComptime::new_default(__resolve_root_defn),
        }
    }
}

pub enum TaskSet {
    Nothing,
    Shutdown,
    SendUpdates,
    Respond(lsp_server::Response),
}

impl TaskSet {
    pub fn then(&mut self, next: TaskSet) {
        match self {
            TaskSet::Nothing => *self = next,
            TaskSet::Shutdown => (),
            TaskSet::SendUpdates => todo!(),
            TaskSet::Respond(_) => todo!(),
        }
    }
}
