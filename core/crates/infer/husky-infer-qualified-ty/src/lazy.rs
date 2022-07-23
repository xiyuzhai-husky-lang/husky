mod value;
mod variable;

pub use value::*;
pub use variable::*;

use crate::*;
use husky_display_utils::{HuskyDisplay, HuskyDisplayConfig};
use infer_error::derived;
use std::fmt::Write;
use word::RootIdentifier;
