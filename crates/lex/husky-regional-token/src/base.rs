use crate::*;

/// equal to the value of TokenIdx::index on the starting token
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenRegionBase(usize);

impl TokenRegionBase {
    pub fn new(token_group_base: TokenGroupStart) -> Self {
        Self(token_group_base.token_idx().index())
    }

    pub(crate) fn index_base(&self) -> usize {
        self.0
    }
}
