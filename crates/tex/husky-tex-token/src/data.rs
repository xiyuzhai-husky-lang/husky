pub mod math;
pub mod text;

use self::{math::TexMathTokenData, text::TexTextTokenData};
use crate::lexer::TexTokenizer;
#[cfg(test)]
use crate::*;
use husky_tex_prelude::mode::TexMode;

#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TexTokenData {
    Math(TexMathTokenData),
    Text(TexTextTokenData),
}

impl<'a> TexTokenizer<'a> {
    pub(crate) fn next_token_data(&mut self) -> Option<TexTokenData> {
        match self.mode {
            TexMode::Text => self.next_text_token_data().map(Into::into),
            TexMode::Math => self.next_math_token_data().map(Into::into),
        }
    }
}
