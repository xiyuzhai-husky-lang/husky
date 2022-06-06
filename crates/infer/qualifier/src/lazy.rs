mod value;
mod variable;

pub use value::*;
pub use variable::*;

use crate::*;
use infer_error::derived;
use std::fmt::Write;
use test_utils::{TestDisplay, TestDisplayConfig};
use word::RootIdentifier;
