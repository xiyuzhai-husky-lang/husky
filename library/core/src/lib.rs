pub mod num;
pub mod ops;
pub mod slice;
pub mod vec;

pub use self::num::*;
pub use self::ops::*;
pub use self::slice::*;
pub use self::vec::*;
pub use ordered_float::NotNan;

use husky_standard_value::ugly::*;

pub type Leash<T> = &'static T;

#[macro_export]
macro_rules! require {
    (let $($tt: tt)*) => {
        let $($tt)* else {
            return Default::default()
        };
    };
    ($condition: expr) => {
        if !($condition) {
            return Default::default()
        }
    }
}

#[macro_export]
macro_rules! unveil {
    ($return_ty: ty, $result: expr, $runtime_const_symbols: expr) => {
        match <$return_ty as Unveil<_>>::unveil($result, $runtime_const_symbols) {
            std::ops::ControlFlow::Break(b) => return b,
            std::ops::ControlFlow::Continue(c) => c,
        }
    };
}
