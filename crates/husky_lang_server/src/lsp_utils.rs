//! Utilities for LSP-related boilerplate code.
use std::error::Error;

pub(crate) fn is_cancelled(_e: &(dyn Error + 'static)) -> bool {
    todo!()
}
