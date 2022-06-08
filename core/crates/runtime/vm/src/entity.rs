use crate::*;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityUid {
    pub raw: usize,
}

impl EntityUid {
    pub fn raw(&self) -> usize {
        self.raw
    }
}
