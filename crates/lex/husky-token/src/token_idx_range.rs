use std::iter::Step;

use crate::*;

#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
pub struct TokenIdxRange {
    start: TokenIdxRangeStart,
    end: TokenIdxRangeEnd,
}

impl IntoIterator for TokenIdxRange {
    type Item = TokenIdx;

    type IntoIter = <core::ops::Range<TokenIdx> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.to_range().clone().into_iter()
    }
}

impl Step for TokenIdx {
    fn steps_between(start: &Self, end: &Self) -> Option<usize> {
        if start.index() <= end.index() {
            Some(end.index() - start.index())
        } else {
            None
        }
    }

    fn forward_checked(start: Self, count: usize) -> Option<Self> {
        Some(start + count)
    }

    fn backward_checked(start: Self, count: usize) -> Option<Self> {
        if start.index() >= count {
            Some(start - count)
        } else {
            None
        }
    }
}

impl TokenIdxRange {
    #[inline(always)]
    pub unsafe fn new(start: TokenIdx, end: TokenIdx) -> Self {
        Self {
            start: TokenIdxRangeStart(start),
            end: TokenIdxRangeEnd(end),
        }
    }

    pub fn to_range(&self) -> std::ops::Range<TokenIdx> {
        self.start.0..self.end.0
    }

    pub(crate) fn from_indices(start: usize, end: usize) -> Self {
        Self {
            start: TokenIdxRangeStart(TokenIdx::from_index(start)),
            end: TokenIdxRangeEnd(TokenIdx::from_index(end)),
        }
    }

    pub fn start(&self) -> TokenIdxRangeStart {
        self.start
    }

    pub fn end(&self) -> TokenIdxRangeEnd {
        self.end
    }

    pub fn new_single(token_idx: TokenIdx) -> Self {
        Self {
            start: TokenIdxRangeStart(token_idx),
            end: TokenIdxRangeEnd(token_idx + 1),
        }
    }
    pub fn new_drained(token_idx: TokenIdx) -> Self {
        Self {
            start: TokenIdxRangeStart(token_idx - 1),
            end: TokenIdxRangeEnd(token_idx),
        }
    }

    pub fn new_closed(first: TokenIdx, last: TokenIdx) -> Self {
        Self {
            start: TokenIdxRangeStart(first),
            end: TokenIdxRangeEnd(last + 1),
        }
    }

    #[inline(always)]
    pub fn to(self, end: TokenIdxRangeEnd) -> Self {
        Self {
            start: self.start,
            end,
        }
    }

    #[inline(always)]
    pub fn join(self, other: TokenIdxRange) -> Self {
        self.to(other.end())
    }
}

impl std::fmt::Debug for TokenIdxRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self.start.0.index()..self.end.0.index()).fmt(f)
    }
}

#[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
pub struct TokenIdxRangeStart(TokenIdx);

impl TokenIdxRangeStart {
    pub fn token_idx(self) -> TokenIdx {
        self.0
    }

    pub fn index(self) -> usize {
        self.0.index()
    }
}

#[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
pub struct TokenIdxRangeEnd(TokenIdx);

impl TokenIdxRangeEnd {
    pub fn new_after(token_idx: TokenIdx) -> Self {
        Self(token_idx + 1)
    }

    pub fn token_idx(self) -> TokenIdx {
        self.0
    }

    pub fn index(self) -> usize {
        self.0.index()
    }
}

impl From<(TokenIdxRangeStart, TokenIdxRangeEnd)> for TokenIdxRange {
    fn from((start, end): (TokenIdxRangeStart, TokenIdxRangeEnd)) -> Self {
        Self { start, end }
    }
}

pub trait HasTokenIdxRange {
    fn token_idx_range(&self) -> TokenIdxRange;
}

impl HasTokenIdxRange for TokenIdxRange {
    fn token_idx_range(&self) -> TokenIdxRange {
        *self
    }
}

impl<T> HasTokenIdxRange for [T]
where
    T: HasTokenIdxRange,
{
    fn token_idx_range(&self) -> TokenIdxRange {
        if self.len() == 0 {
            todo!();
            // return TokenIdxRange {
            //     start: TokenIdxRangeStart(TokenIdx(0)),
            //     end: TokenIdxRangeEnd(TokenIdx(0)),
            // };
        }
        TokenIdxRange {
            start: self.first().unwrap().token_idx_range().start,
            end: self.last().unwrap().token_idx_range().end,
        }
    }
}
