#![feature(const_trait_impl)]
// mod __rust_code_gen__;
mod context;
mod error;
mod linkage;
// #[cfg(test)]
// mod tests;
#[cfg(feature = "thick_fp")]
mod thick_fp;
#[cfg(feature = "thin_fp")]
mod thin_fp;
// mod virtual_enum;
// mod virtual_function;
mod virtual_thick_fp;

// pub use __rust_code_gen__::*;
pub use self::context::*;
pub use self::error::*;
pub use self::linkage::*;
#[cfg(feature = "thick_fp")]
pub use self::thick_fp::*;
#[cfg(feature = "thin_fp")]
pub use self::thin_fp::*;
// pub use self::virtual_enum::*;
// pub use self::virtual_function::*;

pub use husky_regular_value::*;
pub use std::ffi::c_void;

use husky_dev_utils::*;
