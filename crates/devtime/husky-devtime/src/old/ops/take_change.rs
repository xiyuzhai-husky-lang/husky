use crate::*;
use husky_vm::__VMError;
use std::ops::FromResidual;
use trackable::{Trackable, TrackableAtom, TrackableVec};

// #[must_use]
// pub enum DevtimeTakeChangeM<T> {
//     Ok(T),
//     OtherworldlyErr(__VMError),
// }

impl<Task: IsTask> Devtime<Task> {
    pub(crate) fn take_change(&mut self) -> DevtimeStateChange {
        self.state.take_change()
    }
}
