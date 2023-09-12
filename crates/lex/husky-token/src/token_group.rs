use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenGroupStart(TokenIdx);

impl TokenGroupStart {
    pub fn from_index(index: usize) -> Self {
        Self(TokenIdx::from_index(index))
    }

    pub fn token_idx(self) -> TokenIdx {
        self.0
    }

    pub fn index(self) -> usize {
        self.0.index()
    }
}

/// 0-based
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct TokenGroupRelativeTokenIndex(usize);

impl TokenGroupRelativeTokenIndex {
    pub(crate) fn new(base: TokenGroupStart, token_idx: TokenIdx) -> Self {
        debug_assert!(base.token_idx() <= token_idx);
        Self(token_idx.index() - base.index())
    }

    pub(crate) fn index(self) -> usize {
        self.0
    }

    pub(crate) fn token_idx(self, base: TokenGroupStart) -> TokenIdx {
        base.0 + self.0
    }
}

impl std::ops::AddAssign<usize> for TokenGroupRelativeTokenIndex {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs
    }
}

impl std::ops::SubAssign<usize> for TokenGroupRelativeTokenIndex {
    fn sub_assign(&mut self, rhs: usize) {
        self.0 -= rhs
    }
}
