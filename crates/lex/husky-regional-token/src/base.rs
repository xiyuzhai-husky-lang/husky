use crate::*;

/// equal to the value of TokenIdx::index on the starting token
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegionalTokenIdxBase(usize);

impl RegionalTokenIdxBase {
    pub fn new_snippet() -> Self {
        Self(0)
    }

    pub fn new(token_verse_base: TokenVerseStart) -> Self {
        Self(token_verse_base.token_idx().index())
    }

    pub fn index_base(&self) -> usize {
        self.0
    }
}
