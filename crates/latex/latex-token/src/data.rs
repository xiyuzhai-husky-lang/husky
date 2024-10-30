pub mod math;
pub mod rose;

use self::{math::LxMathTokenData, rose::LxRoseTokenData};
use crate::lexer::LxLexer;
#[cfg(test)]
use crate::*;
use latex_prelude::mode::LxMode;

#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LxTokenData {
    Math(LxMathTokenData),
    Rose(LxRoseTokenData),
}
