pub(crate) mod client_comm;
pub(crate) mod event_loop_comm;

use crate::*;
use client_comm::ClientCommunicator;
use crossbeam_channel::Sender;
use event_loop_comm::EventLoopCommunicator;
use threadpool::ThreadPool;

pub(crate) struct Server {
    pub(crate) client_comm: ClientCommunicator,
    pub(crate) event_loop_comm: EventLoopCommunicator,
    pub(crate) db: AnalysisHost,
    pub(crate) threadpool: ThreadPool,
}

impl Server {
    pub fn new(sender: Sender<lsp_server::Message>) -> Server {
        Server {
            client_comm: ClientCommunicator::new(sender),
            threadpool: ThreadPool::default(),
            event_loop_comm: EventLoopCommunicator::default(),
            // ad hoc
            db: AnalysisHost::default(),
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
