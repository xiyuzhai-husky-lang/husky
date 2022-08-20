use crate::*;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TempFeatureUid(usize);

impl Default for TempFeatureUid {
    fn default() -> Self {
        Self(Default::default())
    }
}

static TEMP_FEATURE_UID_NEXT_RAW: AtomicUsize = AtomicUsize::new(0);

impl Feature {
    pub fn new_temp() -> Self {
        Feature::Temp {
            uid: TempFeatureUid(TEMP_FEATURE_UID_NEXT_RAW.fetch_add(1, Ordering::SeqCst)),
        }
    }
}
