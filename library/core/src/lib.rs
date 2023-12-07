pub mod ops;
pub mod vec;

pub use self::ops::*;
pub use self::vec::*;

pub type Leash<T> = &'static T;

#[macro_export]
macro_rules! require {
    (let $($tt: tt)*) => {
        let $($tt)* else {
            return Default::default()
        }
    };
    ($condition: expr) => {
        if !($condition) {
            return Default::default()
        }
    }
}
