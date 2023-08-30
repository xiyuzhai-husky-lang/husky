use crate::*;

#[derive(Debug, Clone, Copy)]
pub struct MinimalDevLinkage(
    extern "C" fn(smallvec::SmallVec<[__RegularValue; 4]>) -> __RegularValue,
);
