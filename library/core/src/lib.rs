pub mod ops;

pub use self::ops::*;

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
