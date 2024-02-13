use super::*;
use shifted_unsigned_int::ShiftedU32;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TokenVerseIdx {
    /// None if the verse is main,
    ///
    /// Some(_) if the verse is nested
    lcurl: Option<TokenIdx>,
    raw: ShiftedU32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TokenVerseIdxRange;

/// # constructor
impl TokenVerseIdx {
    pub(super) fn new(lcurl: Option<TokenIdx>, index: usize) -> Self {
        Self {
            lcurl,
            raw: index.into(),
        }
    }
}

/// # getters
impl TokenVerseIdx {
    pub fn lcurl(self) -> Option<TokenIdx> {
        self.lcurl
    }

    pub fn index(self) -> usize {
        self.raw.index()
    }

    pub(crate) fn next(self) -> Self {
        Self {
            lcurl: self.lcurl,
            raw: unsafe { self.raw.unchecked_add(1) },
        }
    }
}

impl std::fmt::Display for TokenVerseIdx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.raw.index(), f)
    }
}
