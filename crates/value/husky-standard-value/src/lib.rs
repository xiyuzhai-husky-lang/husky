#![feature(trait_upcasting)]
//!
//! Given `struct A { x: &mut T, ys: Vec<i32>, .. }` Regular Value Type
//! - `struct __ThawedA { x: &'static mut T, ys: Vec<i32>, .. }` Regular Value is its Slush version with every lifetime being static,
//! - `struct __SnapshotA { x: Box<T>, ys: Vec<i32>, .. }` is its Snapshot version,
//! which does have extra heap allocation
//! - `struct __IncubatorA { x: Box<T>, ys: ManuallyDrop<Vec<i32>>, .. }` is its Slush version with Drop customized,
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
//!     type __ThawedSelf = __ThawedA;
//! }
//!
//! struct __ThawedA {
//!     x: *mut T,
//!     ys: Vec<i32>,
//! }
//!
//! impl __RegularSlush for __ThawedA {
//!     type __SnapshotSelf = __SnapshotA;
//! }
//! ```

mod boiled;
#[cfg(feature = "constant")]
mod constant;
pub mod exception;
pub mod frozen;
#[cfg(feature = "helpers")]
mod helpers;
// mod regular;
pub mod slush;
pub mod thawed;
#[cfg(feature = "ugly")]
pub mod ugly;
mod value;

use std::num::NonZeroU32;

pub use self::boiled::*;
#[cfg(feature = "constant")]
pub use self::constant::*;
// pub use self::regular::*;
pub use self::value::*;
pub use husky_standard_value_macros::*;

use self::exception::{ExceptedValue, Exception};
use serde_impl::IsSerdeImpl;
use std::{
    panic::{RefUnwindSafe, UnwindSafe},
    sync::Arc,
};

pub type JsonValue = <serde_impl::json::SerdeJson as IsSerdeImpl>::Value;
