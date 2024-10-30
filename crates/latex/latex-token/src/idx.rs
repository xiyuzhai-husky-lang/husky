use shifted_unsigned_int::ShiftedU32;
use std::iter::Step;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct LxTokenIdx(ShiftedU32);

impl LxTokenIdx {
    pub(crate) fn from_index(index: usize) -> LxTokenIdx {
        Self(index.into())
    }

    pub(crate) fn index(self) -> usize {
        self.0.index()
    }
}

impl Add<usize> for LxTokenIdx {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl Sub<usize> for LxTokenIdx {
    type Output = Self;
    fn sub(self, rhs: usize) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl Step for LxTokenIdx {
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

#[derive(Debug)]
pub struct LxTokenIdxRange {
    start: ShiftedU32,
    end: ShiftedU32,
}

impl IntoIterator for LxTokenIdxRange {
    type Item = LxTokenIdx;
    type IntoIter = std::ops::Range<LxTokenIdx>;

    fn into_iter(self) -> Self::IntoIter {
        LxTokenIdx::from_index(self.start.index())..LxTokenIdx::from_index(self.end.index())
    }
}

impl From<std::ops::Range<usize>> for LxTokenIdxRange {
    fn from(range: std::ops::Range<usize>) -> Self {
        Self {
            start: range.start.into(),
            end: range.end.into(),
        }
    }
}

impl LxTokenIdxRange {
    pub(crate) fn new(range: std::ops::Range<usize>) -> Self {
        Self {
            start: range.start.into(),
            end: range.end.into(),
        }
    }

    pub fn new_single(idx: LxTokenIdx) -> Self {
        Self {
            start: idx.0,
            end: (idx + 1).0,
        }
    }
}
