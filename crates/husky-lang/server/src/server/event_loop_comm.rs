use crossbeam_channel::{unbounded, Receiver, Sender};

use super::TaskSet;

pub(crate) struct EventLoopCommunicator {
    pub(crate) sender: Sender<TaskSet>,
    pub(crate) receiver: Receiver<TaskSet>,
}

impl Default for EventLoopCommunicator {
    fn default() -> Self {
        let (sender, receiver) = unbounded();
        Self { sender, receiver }
    }
}
