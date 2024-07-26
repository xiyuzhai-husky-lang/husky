pub use crate::{
    item_path::ItemPathIdInterface as __ItemPathIdInterface,
    ki_repr::{
        KiArgumentReprInterface as __KiArgumentReprInterface,
        KiDomainReprInterface as __KiDomainReprInterface, KiReprInterface as __KiReprInterface,
        KiRuntimeConstantInterface as __ValRuntimeConstantInterface,
    },
    static_var::IsStaticVar as __IsStaticVar,
};
pub use husky_visual_protocol::ugly::*;

pub type __SmallVec<T> = smallvec::SmallVec<T>;
