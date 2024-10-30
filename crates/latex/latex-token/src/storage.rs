use crate::{
    data::LxTokenData,
    idx::{LxTokenIdx, LxTokenIdxRange},
    stream::LxTokenStream,
};
use husky_text_protocol::range::TextRange;

#[derive(Default, Debug)]
pub struct LxTokenStorage {
    ranged_tokens: Vec<((usize, usize), TextRange, LxTokenData)>,
}

/// # getters

impl std::ops::Index<LxTokenIdx> for LxTokenStorage {
    type Output = LxTokenData;

    fn index(&self, idx: LxTokenIdx) -> &Self::Output {
        &self.ranged_tokens[idx.index()].2
    }
}

impl LxTokenStorage {
    pub fn whole_token_idx_range(&self) -> LxTokenIdxRange {
        LxTokenIdxRange::new(0..self.ranged_tokens.len())
    }

    pub fn whole_token_idx_range_without_the_first(&self) -> LxTokenIdxRange {
        LxTokenIdxRange::new(1..self.ranged_tokens.len())
    }

    pub fn token_text_range(&self, token_idx: LxTokenIdx) -> TextRange {
        self.ranged_tokens[token_idx.index()].1
    }

    pub fn stream(&self) -> LxTokenStream {
        LxTokenStream::new(&self.ranged_tokens)
    }
}

/// # actions

impl LxTokenStorage {
    pub(crate) fn alloc(
        &mut self,
        start_offset: usize,
        end_offset: usize,
        range: TextRange,
        token_data: LxTokenData,
    ) -> LxTokenIdx {
        let idx = LxTokenIdx::from_index(self.ranged_tokens.len());
        self.ranged_tokens
            .push(((start_offset, end_offset), range, token_data));
        idx
    }
}
