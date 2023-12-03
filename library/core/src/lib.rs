pub mod ops;

pub use self::ops::*;

pub type Leash<T> = &'static T;
