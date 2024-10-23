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

impl<'a> LxLexer<'a> {
    pub(crate) fn next_token_data(&mut self) -> Option<LxTokenData> {
        match self.mode {
            LxMode::Rose => self.next_text_token_data().map(Into::into),
            LxMode::Math => self.next_math_token_data().map(Into::into),
        }
    }
}
