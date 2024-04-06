pub mod code;
pub mod math;
pub mod rose;

use self::{code::TexCodeTokenData, math::TexMathTokenData, rose::TexRoseTokenData};
use crate::lexer::TexLexer;
#[cfg(test)]
use crate::*;
use husky_tex_prelude::mode::TexMode;

#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TexTokenData {
    Code(TexCodeTokenData),
    Math(TexMathTokenData),
    Rose(TexRoseTokenData),
}

impl<'a> TexLexer<'a> {
    pub(crate) fn next_token_data(&mut self) -> Option<TexTokenData> {
        match self.mode {
            TexMode::Code => todo!(),
            TexMode::Rose => self.next_text_token_data().map(Into::into),
            TexMode::Math => self.next_math_token_data().map(Into::into),
        }
    }
}
