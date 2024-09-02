use crate::*;
use frozen::FrozenDyn;
use smallvec::SmallVec;

pub enum SlushValue {
    Box(Box<dyn std::any::Any>),
    Arc(Arc<dyn FrozenDyn>),
}

pub type SlushValues = SmallVec<[SlushValue; 8]>;
