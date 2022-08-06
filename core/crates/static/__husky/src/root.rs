mod b32x;
pub mod domains;
mod f32x;
mod i32x;
mod vecx;

pub use b32x::*;
pub use f32x::*;
pub use husky_vm_interface::{
    __EvalContext, __Register, __Registrable, __StaticInfo, entity_uid, feature_ptr,
};
pub use i32x::*;
pub use vecx::*;

pub mod __std {
    pub mod slice {
        pub use cyclic_slice::CyclicSlice;
    }
}

#[macro_export]
macro_rules! require {
    ($condition: expr) => {
        if !$condition {
            return None;
        }
    };
}
pub use require;
