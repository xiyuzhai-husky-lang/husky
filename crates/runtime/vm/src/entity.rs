use crate::*;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityUid {
    raw: usize,
}

static ENTITY_NEXT_RAW_ID: AtomicUsize = AtomicUsize::new(0);

impl EntityUid {
    pub fn new() -> EntityUid {
        let raw = ENTITY_NEXT_RAW_ID.fetch_add(1, Ordering::Relaxed);
        EntityUid { raw }
    }

    pub fn raw(&self) -> usize {
        self.raw
    }
}
