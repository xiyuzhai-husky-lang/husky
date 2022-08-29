#![feature(const_trait_impl)]
mod __rust_code_gen__;
mod context;
mod error;
mod extra;
mod linkage;
mod register;
#[cfg(test)]
mod tests;
#[cfg(feature = "thick_fp")]
mod thick_fp;
#[cfg(feature = "thin_fp")]
mod thin_fp;
mod virtual_enum;
mod virtual_function;
mod virtual_thick_fp;

pub use __rust_code_gen__::*;
pub use context::*;
pub use error::*;
pub use linkage::*;
pub use register::*;
pub use std::ffi::c_void;
#[cfg(feature = "thick_fp")]
pub use thick_fp::*;
#[cfg(feature = "thin_fp")]
pub use thin_fp::*;
pub use virtual_enum::*;
pub use virtual_function::*;
pub use virtual_thick_fp::*;

use husky_dev_utils::*;
