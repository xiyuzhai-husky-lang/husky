use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegionalTokenVerseStart(RegionalTokenIdx);

impl RegionalTokenVerseStart {
    pub fn from_token_verse_start(
        token_verse_start: TokenVerseStart,
        regional_token_idx_base: RegionalTokenIdxBase,
    ) -> Self {
        let index = token_verse_start.index() - regional_token_idx_base.index_base();
        Self::from_index(index)
    }

    pub(crate) fn from_index(index: usize) -> Self {
        Self(RegionalTokenIdx::from_index(index))
    }

    pub fn regional_token_idx(self) -> RegionalTokenIdx {
        self.0
    }

    pub fn index(self) -> usize {
        self.0.index()
    }
}
