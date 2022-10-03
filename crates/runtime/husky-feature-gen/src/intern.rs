use husky_feature_protocol::FeatureId;
use interner::IsInternPtr;

use crate::*;

pub trait InternFeature {
    fn feature_interner(&self) -> &FeatureInterner;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FeaturePtr {
    id: FeatureId,
}

impl FeaturePtr {
    pub unsafe fn from_raw(raw: usize) -> Self {
        Self {
            id: FeatureId::new(raw),
        }
    }

    pub fn id(self) -> FeatureId {
        self.id
    }
}

impl std::ops::Deref for FeaturePtr {
    type Target = Feature;

    fn deref(&self) -> &Self::Target {
        unreachable!()
    }
}

impl std::borrow::Borrow<Feature> for FeaturePtr {
    fn borrow(&self) -> &Feature {
        unreachable!()
    }
}

impl IsInternPtr for FeaturePtr {
    type T = Feature;

    type Owned = Feature;

    fn new_intern_ptr(id: usize, feature: &'static Self::T) -> Self {
        Self {
            id: FeatureId::new(id),
        }
    }
}

pub type FeatureInterner = interner::Interner<FeaturePtr>;

pub fn new_feature_interner() -> FeatureInterner {
    FeatureInterner::new(&[])
}
