use crate::*;

pub trait AllocateUniqueFeature {
    fn feature_interner(&self) -> &FeatureInterner;
}

pub type FeatureInterner = unique_allocator::UniqueAllocator<Feature>;

pub type FeaturePtr = unique_allocator::InternedPtr<Feature>;

pub fn new_feature_interner() -> FeatureInterner {
    FeatureInterner::new(&[])
}
