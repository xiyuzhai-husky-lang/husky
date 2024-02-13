use crate::*;

pub struct RegionalTokenVerseData {}

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

/// 0-based
///
/// the relative token idx in a regional token verse
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct RegionalTokenVerseRelativeTokenIndex(usize);

impl RegionalTokenVerseRelativeTokenIndex {
    pub(crate) fn new(base: RegionalTokenVerseStart, regional_token_idx: RegionalTokenIdx) -> Self {
        debug_assert!(base.regional_token_idx() <= regional_token_idx);
        Self(regional_token_idx.index() - base.index())
    }

    pub(crate) fn index(self) -> usize {
        self.0
    }

    pub(crate) fn regional_token_idx(self, base: RegionalTokenVerseStart) -> RegionalTokenIdx {
        base.0 + self.0
    }
}

impl std::ops::AddAssign<usize> for RegionalTokenVerseRelativeTokenIndex {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs
    }
}

impl std::ops::SubAssign<usize> for RegionalTokenVerseRelativeTokenIndex {
    fn sub_assign(&mut self, rhs: usize) {
        self.0 -= rhs
    }
}
