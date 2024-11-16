pub mod lisp;
pub mod math;
pub mod root;
pub mod rose;
pub mod word;

use self::{
    lisp::LxLispTokenData, math::LxMathTokenData, root::LxRootTokenData, rose::LxRoseTokenData,
    word::LxWordTokenData,
};
use crate::lexer::LxLexer;
#[cfg(test)]
use crate::*;
use latex_prelude::mode::LxMode;

#[enum_class::from_variants]
#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LxTokenData {
    Lisp(LxLispTokenData),
    Math(LxMathTokenData),
    Root(LxRootTokenData),
    Rose(LxRoseTokenData),
    Word(LxWordTokenData),
}
