use crate::*;

pub trait InternFeature {
    fn feature_interner(&self) -> &FeatureInterner;
}

pub type FeatureInterner = interner::Interner<Feature>;

pub type FeaturePtr = interner::InternedPtr<Feature>;

pub fn new_feature_interner() -> FeatureInterner {
    FeatureInterner::new(&[])
}
