use crate::{data::LxTokenData, idx::LxTokenIdx};
use husky_text_protocol::range::TextRange;

pub struct LxTokenStream<'a> {
    ranged_tokens: &'a [((usize, usize), TextRange, LxTokenData)],
    current_idx: usize,
}

impl<'a> LxTokenStream<'a> {
    pub(crate) fn new(ranged_tokens: &'a [((usize, usize), TextRange, LxTokenData)]) -> Self {
        Self {
            ranged_tokens,
            current_idx: 0,
        }
    }
}

impl<'a> Iterator for LxTokenStream<'a> {
    type Item = (LxTokenIdx, (usize, usize), TextRange, LxTokenData);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_idx >= self.ranged_tokens.len() {
            return None;
        }
        let idx = self.current_idx;
        let (offset_range, text_range, data) = self.ranged_tokens[idx];
        self.current_idx += 1;
        Some((LxTokenIdx::from_index(idx), offset_range, text_range, data))
    }
}
