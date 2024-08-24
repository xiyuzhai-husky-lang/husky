#![feature(try_trait_v2)]
#![feature(try_trait_v2_residual)]
pub mod anchor;
pub mod chart;
pub mod devsoul;
pub mod ugly;

pub use husky_devsoul_interface_macros::*;

use husky_value_interface::IsValue;
use once_cell::sync::OnceCell;
use shifted_unsigned_int::ShiftedU32;
use std::convert::Infallible;
