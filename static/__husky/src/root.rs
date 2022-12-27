pub mod domains;
mod f32x;
mod i32x;
mod r32x;
mod vecx;

pub use f32x::*;
pub use husky_vm_interface::{
    __EvalContext, __Register, __Registrable, __StaticInfo, entity_uid, feature_ptr, ThickFp,
};
pub use i32x::*;
pub use r32x::*;
pub use vecx::*;

pub mod __std {
    pub mod slice {
        pub use cyclic_slice::CyclicSlice;
    }
}

#[macro_export]
macro_rules! normal_require {
    ($condition: expr) => {
        if !$condition {
            return None;
        }
    };
}
pub use normal_require;

#[macro_export]
macro_rules! feature_require {
    ($ctx: expr, $feature: expr, $condition: expr) => {
        if !$condition {
            $ctx.cache_feature($feature, Ok(__Register::none()));
            return None;
        }
    };
}
pub use feature_require;
