#![allow(warnings)]

mod error;
mod session;
mod tests;

pub use error::{RuntimeError, RuntimeResult, RuntimeResultArc};

use interpret::{interpret, Any, Instruction};
