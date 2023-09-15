use crate::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct RegionalTokenIdxRange {
    start: RegionalTokenIdxRangeStart,
    end: RegionalTokenIdxRangeEnd,
}

impl RegionalTokenIdxRange {
    pub fn from_token_idx_range(
        token_idx_range: TokenIdxRange,
        regional_token_idx_base: RegionalTokenIdxBase,
    ) -> Self {
        Self {
            start: RegionalTokenIdxRangeStart(RegionalTokenIdx::from_token_idx(
                token_idx_range.start().token_idx(),
                regional_token_idx_base,
            )),
            end: RegionalTokenIdxRangeEnd(RegionalTokenIdx::from_token_idx(
                token_idx_range.end().token_idx(),
                regional_token_idx_base,
            )),
        }
    }

    pub fn token_idx_range(self, base: RegionalTokenIdxBase) -> TokenIdxRange {
        let start = self.start.regional_token_idx().token_idx(base);
        let end = self.end.regional_token_idx().token_idx(base);
        unsafe { TokenIdxRange::new(start, end) }
    }

    #[inline(always)]
    pub fn new(start: RegionalTokenIdx, end: RegionalTokenIdxRangeEnd) -> Self {
        Self {
            start: RegionalTokenIdxRangeStart(start),
            end,
        }
    }

    pub(crate) fn from_indices(start: usize, end: usize) -> Self {
        Self {
            start: RegionalTokenIdxRangeStart(RegionalTokenIdx::from_index(start)),
            end: RegionalTokenIdxRangeEnd(RegionalTokenIdx::from_index(end)),
        }
    }

    pub fn start(&self) -> RegionalTokenIdxRangeStart {
        self.start
    }

    pub fn end(&self) -> RegionalTokenIdxRangeEnd {
        self.end
    }

    pub fn new_single(regional_token_idx: RegionalTokenIdx) -> Self {
        Self {
            start: RegionalTokenIdxRangeStart(regional_token_idx),
            end: RegionalTokenIdxRangeEnd(regional_token_idx + 1),
        }
    }
    pub fn new_drained(regional_token_idx: RegionalTokenIdx) -> Self {
        Self {
            start: RegionalTokenIdxRangeStart(regional_token_idx - 1),
            end: RegionalTokenIdxRangeEnd(regional_token_idx),
        }
    }

    pub fn new_closed(first: RegionalTokenIdx, last: RegionalTokenIdx) -> Self {
        Self {
            start: RegionalTokenIdxRangeStart(first),
            end: RegionalTokenIdxRangeEnd(last + 1),
        }
    }

    #[inline(always)]
    pub fn to(self, end: RegionalTokenIdxRangeEnd) -> Self {
        Self {
            start: self.start,
            end,
        }
    }

    #[inline(always)]
    pub fn join(self, other: RegionalTokenIdxRange) -> Self {
        self.to(other.end())
    }
}

impl std::fmt::Debug for RegionalTokenIdxRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self.start.0.index()..self.end.0.index()).fmt(f)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegionalTokenIdxRangeStart(RegionalTokenIdx);

impl RegionalTokenIdxRangeStart {
    pub fn regional_token_idx(self) -> RegionalTokenIdx {
        self.0
    }

    pub fn index(self) -> usize {
        self.0.index()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegionalTokenIdxRangeEnd(RegionalTokenIdx);

impl RegionalTokenIdxRangeEnd {
    pub fn new_after(regional_token_idx: RegionalTokenIdx) -> Self {
        Self(regional_token_idx + 1)
    }

    pub fn regional_token_idx(self) -> RegionalTokenIdx {
        self.0
    }

    pub fn index(self) -> usize {
        self.0.index()
    }
}
