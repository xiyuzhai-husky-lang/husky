pub mod num;
pub mod ops;
pub mod slice;
pub mod vec;

pub use self::num::*;
pub use self::ops::*;
pub use self::slice::*;
pub use self::vec::*;
pub use ordered_float::NotNan;

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
