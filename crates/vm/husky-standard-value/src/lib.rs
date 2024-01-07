//!
//! Given `struct A { x: &mut T, ys: Vec<i32>, .. }` Regular Value Type
//! - `struct __StaticA { x: &'static mut T, ys: Vec<i32>, .. }` Regular Value is its Stand version with every lifetime being static,
//! - `struct __SnapshotA { x: Box<T>, ys: Vec<i32>, .. }` is its Snapshot version,
//! which does have extra heap allocation
//! - `struct __IncubatorA { x: Box<T>, ys: ManuallyDrop<Vec<i32>>, .. }` is its Stand version with Drop customized,
//! which does have extra heap allocation
//!
//! All four types have the same size
//!
//! ```rust_todo
//! struct A {
//!     x: &mut T,
//!     ys: Vec<i32>,
//! }
//!
//! impl __Regular for A {
//!     type __StaticSelf = __StaticA;
//! }
//!
//! struct __StaticA {
//!     x: *mut T,
//!     ys: Vec<i32>,
//! }
//!
//! impl __RegularStand for __StaticA {
//!     type __SnapshotSelf = __SnapshotA;
//! }
//! ```

#[cfg(feature = "constant")]
mod constant;
pub mod frozen;
#[cfg(feature = "helpers")]
mod helpers;
mod regular;
mod r#static;
pub mod ugly;
mod value;
mod weak_static;

use std::num::NonZeroU32;

#[cfg(feature = "constant")]
pub use self::constant::*;
pub use self::regular::*;
pub use self::value::*;
pub use self::weak_static::*;
pub use husky_standard_value_macros::*;

#[cfg(feature = "constant")]
use husky_term_prelude::*;
use std::{
    panic::{RefUnwindSafe, UnwindSafe},
    sync::Arc,
};
