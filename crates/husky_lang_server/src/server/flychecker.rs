use crossbeam_channel::{unbounded, Receiver, Sender};

pub(crate) struct FlyChecker {
    pub(crate) receiver: Receiver<flycheck::Message>,
}

impl FlyChecker {
    pub(crate) fn new() -> FlyChecker {
        let (flycheck_sender, flycheck_receiver) = unbounded();
        FlyChecker {
            receiver: flycheck_receiver,
        }
    }
}
