use shifted_unsigned_int::ShiftedU32;
use std::iter::Step;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct LxMathTokenIdx(ShiftedU32);

impl LxMathTokenIdx {
    pub(crate) fn from_index(index: usize) -> LxMathTokenIdx {
        Self(index.into())
    }

    pub(crate) fn index(self) -> usize {
        self.0.index()
    }
}

impl Add<usize> for LxMathTokenIdx {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl Sub<usize> for LxMathTokenIdx {
    type Output = Self;
    fn sub(self, rhs: usize) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl Step for LxMathTokenIdx {
    fn steps_between(start: &Self, end: &Self) -> Option<usize> {
        end.0.index().checked_sub(start.0.index()).map(|diff| diff)
    }
    fn forward_checked(start: Self, count: usize) -> Option<Self> {
        start.0.checked_add(count.try_into().ok()?).map(Self)
    }
    fn backward_checked(start: Self, count: usize) -> Option<Self> {
        start.0.checked_sub(count.try_into().ok()?).map(Self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LxMathTokenIdxRange {
    start: ShiftedU32,
    end: ShiftedU32,
}

impl IntoIterator for LxMathTokenIdxRange {
    type Item = LxMathTokenIdx;
    type IntoIter = std::ops::Range<LxMathTokenIdx>;

    fn into_iter(self) -> Self::IntoIter {
        LxMathTokenIdx::from_index(self.start.index())..LxMathTokenIdx::from_index(self.end.index())
    }
}

impl From<std::ops::Range<usize>> for LxMathTokenIdxRange {
    fn from(range: std::ops::Range<usize>) -> Self {
        Self {
            start: range.start.into(),
            end: range.end.into(),
        }
    }
}

impl LxMathTokenIdxRange {
    pub(crate) fn new(range: std::ops::Range<usize>) -> Self {
        Self {
            start: range.start.into(),
            end: range.end.into(),
        }
    }

    pub fn new_single(idx: LxMathTokenIdx) -> Self {
        Self {
            start: idx.0,
            end: (idx + 1).0,
        }
    }

    pub(crate) fn is_empty(self) -> bool {
        self.start == self.end
    }

    pub fn start(&self) -> LxMathTokenIdx {
        LxMathTokenIdx(self.start)
    }

    pub fn last(&self) -> Option<LxMathTokenIdx> {
        if self.is_empty() {
            None
        } else {
            Some(LxMathTokenIdx(self.end - 1))
        }
    }
}
