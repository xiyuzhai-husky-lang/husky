#![feature(trait_upcasting)]
//!
//! Given struct A { x: &mut T, ys: Vec<i32>, .. } Regular Value Type
//! - struct __StandA { x: *mut T, ys: Vec<i32>, .. } Regular Value is its Stand version,
//! which doesn't have extra heap allocation
//! - struct __SnapshotA { x: Box<T>, ys: Vec<i32>, .. } is its Snapshot version,
//! which does have extra heap allocation
//! - struct __StandA { x: Box<T>, ys: ManuallyDrop<Vec<i32>>, .. } is its Stand version with Drop customized,
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
//!     type __StandSelf = __StandA;
//! }
//!
//! struct __StandA {
//!     x: *mut T,
//!     ys: Vec<i32>,
//! }
//!
//! impl __RegularStand for __StandA {
//!     type __SnapshotSelf = __SnapshotA;
//! }
//! ```

#[cfg(feature = "constant")]
mod constant;
mod downcast;
#[cfg(feature = "helpers")]
mod helpers;
mod regular;
mod static_info;
mod trivial_impl;
mod upcast;
mod value;

use std::num::NonZeroU32;

#[cfg(feature = "constant")]
pub use self::constant::*;
pub use self::regular::*;
pub use self::static_info::*;
pub use self::trivial_impl::*;
pub use self::value::*;

#[cfg(feature = "constant")]
use husky_term_prelude::*;
use std::{
    panic::{RefUnwindSafe, UnwindSafe},
    sync::Arc,
};
