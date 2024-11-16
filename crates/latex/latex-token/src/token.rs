pub mod coword;
pub mod lisp;
pub mod math;
pub mod root;
pub mod rose;

use self::{
    coword::LxCowordTokenData, lisp::LxLispTokenData, math::LxMathTokenData, root::LxRootTokenData,
    rose::LxRoseTokenData,
};
use crate::lexer::LxLexer;
#[cfg(test)]
use crate::*;
use latex_prelude::mode::LxMode;

#[enum_class::from_variants]
#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LxTokenData {
    Coword(LxCowordTokenData),
    Lisp(LxLispTokenData),
    Math(LxMathTokenData),
    Root(LxRootTokenData),
    Rose(LxRoseTokenData),
}
