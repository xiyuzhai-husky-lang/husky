use crossbeam_channel::{unbounded, Receiver, Sender};
use flycheck::FlycheckHandle;

pub(crate) struct FlyChecker {
    pub(crate) sender: Sender<flycheck::Message>,
    pub(crate) receiver: Receiver<flycheck::Message>,
    pub(crate) handles: Vec<FlycheckHandle>,
}

impl FlyChecker {
    pub(crate) fn new() -> FlyChecker {
        let (sender, receiver) = unbounded();
        FlyChecker {
            sender,
            receiver,
            handles: vec![],
        }
    }
}
