pub use crate::{
    val_repr::{
        ValArgumentReprInterface as __ValArgumentReprInterface,
        ValDomainReprInterface as __ValDomainReprInterface, ValReprInterface as __ValReprInterface,
        ValRuntimeConstantInterface as __ValRuntimeConstantInterface,
    },
    TaskIngredientIndex as __TaskIngredientIndex, TaskJarIndex as __TaskJarIndex,
    TaskJarIndexOnceCell as __TaskJarIndexOnceCell,
};
pub use husky_visual_protocol::ugly::*;

pub type __SmallVec<T> = smallvec::SmallVec<T>;
