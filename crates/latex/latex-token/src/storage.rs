use crate::{
    data::{math::LxMathTokenData, rose::LxRoseTokenData},
    idx::{
        math::{LxMathTokenIdx, LxMathTokenIdxRange},
        rose::{LxRoseTokenIdx, LxRoseTokenIdxRange},
    },
};
use husky_text_protocol::range::TextRange;

#[salsa::derive_debug_with_db]
#[derive(Default, Debug)]
pub struct LxTokenStorage {
    ranged_math_tokens: Vec<((usize, usize), TextRange, LxMathTokenData)>,
    ranged_rose_tokens: Vec<((usize, usize), TextRange, LxRoseTokenData)>,
}

/// # getters

impl std::ops::Index<LxMathTokenIdx> for LxTokenStorage {
    type Output = LxMathTokenData;

    fn index(&self, idx: LxMathTokenIdx) -> &Self::Output {
        &self.ranged_math_tokens[idx.index()].2
    }
}

impl LxTokenStorage {
    pub fn whole_math_token_idx_range(&self) -> LxMathTokenIdxRange {
        LxMathTokenIdxRange::new(0..self.ranged_math_tokens.len())
    }

    pub fn whole_rose_token_idx_range(&self) -> LxRoseTokenIdxRange {
        LxRoseTokenIdxRange::new(0..self.ranged_rose_tokens.len())
    }

    pub fn math_token_offset_range(&self, token_idx: LxMathTokenIdx) -> (usize, usize) {
        self.ranged_math_tokens[token_idx.index()].0
    }

    pub fn rose_token_offset_range(&self, token_idx: LxRoseTokenIdx) -> (usize, usize) {
        self.ranged_rose_tokens[token_idx.index()].0
    }

    pub fn math_token_text_range(&self, token_idx: LxMathTokenIdx) -> TextRange {
        self.ranged_math_tokens[token_idx.index()].1
    }

    pub fn rose_token_text_range(&self, token_idx: LxRoseTokenIdx) -> TextRange {
        self.ranged_rose_tokens[token_idx.index()].1
    }

    pub fn math_token_idx_range_offset_range(&self, range: LxMathTokenIdxRange) -> (usize, usize) {
        let first = self.math_token_offset_range(range.start());
        match range.last() {
            Some(last) => {
                let last = self.math_token_offset_range(last);
                (first.0, last.1)
            }
            None => todo!(),
        }
    }

    pub fn rose_token_idx_range_offset_range(&self, range: LxRoseTokenIdxRange) -> (usize, usize) {
        let first = self.rose_token_offset_range(range.start());
        match range.last() {
            Some(last) => {
                let last = self.rose_token_offset_range(last);
                (first.0, last.1)
            }
            None => todo!(),
        }
    }
}

/// # actions

impl LxTokenStorage {
    pub(crate) fn alloc_math_token(
        &mut self,
        start_offset: usize,
        end_offset: usize,
        range: TextRange,
        token_data: LxMathTokenData,
    ) -> LxMathTokenIdx {
        let idx = LxMathTokenIdx::from_index(self.ranged_math_tokens.len());
        self.ranged_math_tokens
            .push(((start_offset, end_offset), range, token_data));
        idx
    }

    pub(crate) fn alloc_rose_token(
        &mut self,
        start_offset: usize,
        end_offset: usize,
        range: TextRange,
        token_data: LxRoseTokenData,
    ) -> LxRoseTokenIdx {
        let idx = LxRoseTokenIdx::from_index(self.ranged_rose_tokens.len());
        self.ranged_rose_tokens
            .push(((start_offset, end_offset), range, token_data));
        idx
    }
}
