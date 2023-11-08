use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FeatureEvalId(pub(crate) usize);

static NEXT_RAW_ID: AtomicUsize = AtomicUsize::new(0);

impl Default for FeatureEvalId {
    fn default() -> Self {
        Self(NEXT_RAW_ID.fetch_add(1, Ordering::SeqCst))
    }
}
