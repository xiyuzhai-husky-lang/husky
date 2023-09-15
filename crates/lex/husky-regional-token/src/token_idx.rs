use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct RegionalTokenIdx(NonZeroU32);

impl RegionalTokenIdx {
    pub fn index(self) -> usize {
        self.0.get() as usize - 1
    }

    pub fn token_idx(self, base: RegionalTokenIdxBase) -> TokenIdx {
        unsafe { TokenIdx::from_usize_index_ext(self.index() + base.index_base()) }
    }

    pub(crate) fn from_index(index: usize) -> Self {
        debug_assert!(index < (u32::MAX - 1) as usize);
        unsafe { Self(NonZeroU32::new_unchecked((index + 1) as u32)) }
    }

    pub(crate) fn from_token_idx(
        next_token_idx: TokenIdx,
        regional_token_idx_base: RegionalTokenIdxBase,
    ) -> RegionalTokenIdx {
        Self::from_index(next_token_idx.index() - regional_token_idx_base.index_base())
    }
}

impl std::ops::Add<usize> for RegionalTokenIdx {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self::from_index(self.index() + rhs)
    }
}

impl std::ops::Sub<usize> for RegionalTokenIdx {
    type Output = Self;

    fn sub(self, rhs: usize) -> Self::Output {
        Self::from_index(self.index() - rhs)
    }
}
