use crate::{data::TexTokenData, idx::TexTokenIdx};
use husky_text_protocol::range::TextRange;

#[derive(Default)]
pub struct TexTokenStorage {
    ranged_tokens: Vec<(TextRange, TexTokenData)>,
}

/// # getters

impl std::ops::Index<TexTokenIdx> for TexTokenStorage {
    type Output = TexTokenData;

    fn index(&self, idx: TexTokenIdx) -> &Self::Output {
        &self.ranged_tokens[idx.index()].1
    }
}
impl TexTokenStorage {
    pub fn token_range(&self, token_idx: TexTokenIdx) -> TextRange {
        self.ranged_tokens[token_idx.index()].0
    }
}

/// # actions

impl TexTokenStorage {
    pub(crate) fn alloc(&mut self, range: TextRange, token_data: TexTokenData) -> TexTokenIdx {
        let idx = TexTokenIdx::from_index(self.ranged_tokens.len());
        self.ranged_tokens.push((range, token_data));
        idx
    }
}
