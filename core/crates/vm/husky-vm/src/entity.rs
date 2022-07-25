use crate::*;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityUid(usize);

impl EntityUid {
    pub unsafe fn from_raw(raw: usize) -> EntityUid {
        Self(raw)
    }

    pub fn raw(&self) -> usize {
        self.0
    }
}
