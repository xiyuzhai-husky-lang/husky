use crate::*;

pub trait AllocateUniqueFeature {
    fn features(&self) -> &FeatureUniqueAllocator;
}

pub type FeatureUniqueAllocator = unique_allocator::UniqueAllocator<Feature>;

pub type FeaturePtr = unique_allocator::InternedPtr<Feature>;

pub fn new_feature_unique_allocator() -> FeatureUniqueAllocator {
    FeatureUniqueAllocator::new(&[])
}
