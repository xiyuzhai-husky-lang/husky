use husky_token::TokenGroupBase;

use super::*;
use std::num::NonZeroU32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegionalTokenIdx(NonZeroU32);

impl RegionalTokenIdx {
    pub fn index(self) -> usize {
        self.0.get() as usize - 1
    }

    pub fn token_idx(self, base: TokenRegionBase) -> TokenIdx {
        unsafe { TokenIdx::from_usize_index_ext(self.index() + base.0) }
    }
}

/// equal to the value of TokenIdx::index on the starting token
pub struct TokenRegionBase(usize);

impl TokenRegionBase {
    pub(crate) fn new(token_group_base: TokenGroupBase) -> Self {
        Self(token_group_base.token_idx().index())
    }
}
