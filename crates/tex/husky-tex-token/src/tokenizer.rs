mod math;
mod text;

use crate::{
    data::{math::TexMathTokenData, text::TexTextTokenData, TexTokenData},
    idx::TexTokenIdx,
    storage::TexTokenStorage,
    *,
};
use husky_tex_prelude::mode::TexMode;
use husky_text_protocol::{char_iter::TextCharIter, range::TextRange};

pub struct TexTokenizer<'a> {
    db: &'a ::salsa::Db,
    chars: TextCharIter<'a>,
    mode: TexMode,
    storage: TexTokenStorage,
}

impl<'a> Iterator for TexTokenizer<'a> {
    type Item = (TexTokenIdx, TexTokenData);

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.chars.current_position();
        let token_data = self.next_token_data()?;
        let range = TextRange {
            start,
            end: self.chars.current_position(),
        };
        Some((self.storage.alloc(range, token_data), token_data))
    }
}

impl<'a> TexTokenizer<'a> {
    fn next_token_data(&mut self) -> Option<TexTokenData> {
        match self.mode {
            TexMode::Text => self.next_text_token_data().map(Into::into),
            TexMode::Math => self.next_math_token_data().map(Into::into),
        }
    }

    fn new(db: &'a ::salsa::Db, input: &'a str, mode: TexMode) -> Self {
        Self {
            db,
            chars: TextCharIter::new(input),
            mode,
            storage: Default::default(),
        }
    }
}
