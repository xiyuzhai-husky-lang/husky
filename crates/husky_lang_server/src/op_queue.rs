//! Bookkeeping to make sure only one long-running operation is being executed
//! at a time.

pub(crate) struct OpnQueue<Output> {
    opn_requested: bool,
    opn_in_progress: bool,
    last_opn_result: Output,
}

impl<Output: Default> Default for OpnQueue<Output> {
    fn default() -> Self {
        Self {
            opn_requested: false,
            opn_in_progress: false,
            last_opn_result: Default::default(),
        }
    }
}

impl<Output> OpnQueue<Output> {
    pub(crate) fn request_op(&mut self) {
        self.opn_requested = true;
    }
    pub(crate) fn should_start_op(&mut self) -> bool {
        if self.opn_in_progress {
            return false;
        }
        self.opn_in_progress = self.opn_requested;
        self.opn_requested = false;
        self.opn_in_progress
    }
    pub(crate) fn opn_completed(&mut self, result: Output) {
        assert!(self.opn_in_progress);
        self.opn_in_progress = false;
        self.last_opn_result = result;
    }

    pub(crate) fn last_opn_result(&self) -> &Output {
        &self.last_opn_result
    }
    pub(crate) fn opn_in_progress(&self) -> bool {
        self.opn_in_progress
    }
    pub(crate) fn opn_requested(&self) -> bool {
        self.opn_requested
    }
}
