use crossbeam_channel::{unbounded, Receiver, Sender};

use crate::{config::ServerConfig, task::Task, taskpool::TaskPool};

use super::handle::Handle;

pub(super) type ServerTaskPool = Handle<TaskPool<Task>, Receiver<Task>>;

impl ServerTaskPool {
    pub(super) fn new() -> ServerTaskPool {
        let (sender, receiver) = unbounded();
        let handle = TaskPool::new(sender);
        Handle { handle, receiver }
    }
}
