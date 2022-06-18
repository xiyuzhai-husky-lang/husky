use crate::*;

pub trait AllocateUniqueFeature {
    fn features(&self) -> &FeatureInterner;
}

pub type FeatureInterner = unique_allocator::UniqueAllocator<Feature>;

pub type FeaturePtr = unique_allocator::InternedPtr<Feature>;

pub fn new_feature_unique_allocator() -> FeatureInterner {
    FeatureInterner::new(&[])
}
