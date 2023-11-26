use std::num::NonZeroU32;

use crate::*;

/// is eof if raw is equal to the len of all tokens
#[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = TokenDb, jar = TokenJar)]
pub struct TokenIdx(NonZeroU32);

impl TokenIdx {
    pub unsafe fn from_usize_index_ext(index: usize) -> Self {
        debug_assert!(index < (u32::MAX - 1) as usize);
        Self(NonZeroU32::new_unchecked((index + 1) as u32))
    }

    pub(crate) fn from_index(index: usize) -> Self {
        debug_assert!(index < (u32::MAX - 1) as usize);
        unsafe { Self(NonZeroU32::new_unchecked((index + 1) as u32)) }
    }

    pub fn index(self) -> usize {
        (self.0.get() - 1) as usize
    }
}

impl std::ops::Add<usize> for TokenIdx {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self::from_index(self.index() + rhs)
    }
}

impl std::ops::Sub<usize> for TokenIdx {
    type Output = Self;

    fn sub(self, rhs: usize) -> Self::Output {
        Self::from_index(self.index() - rhs)
    }
}
