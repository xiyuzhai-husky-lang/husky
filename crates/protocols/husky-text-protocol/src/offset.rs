use std::ops::{Range, RangeFrom, RangeInclusive, RangeTo, RangeToInclusive};

use serde::{Deserialize, Serialize};
use shifted_unsigned_int::ShiftedU32;

/// Character byte offset (zero-based). The meaning of this
/// offset is determined by the negotiated `PositionEncodingKind`.
#[derive(
    Debug, PartialEq, Default, Eq, PartialOrd, Ord, Clone, Copy, Hash, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct Offset(ShiftedU32);

impl Offset {
    pub fn index(self) -> usize {
        self.0.index()
    }
}

impl From<i32> for Offset {
    fn from(raw: i32) -> Self {
        Offset(ShiftedU32::from(raw))
    }
}

impl From<usize> for Offset {
    fn from(raw: usize) -> Self {
        Offset(ShiftedU32::from(raw))
    }
}

impl From<u32> for Offset {
    fn from(raw: u32) -> Self {
        Offset(ShiftedU32::from(raw))
    }
}

impl std::ops::Add<usize> for Offset {
    type Output = Offset;

    fn add(self, rhs: usize) -> Self::Output {
        Offset(self.0 + rhs)
    }
}

impl std::ops::AddAssign<usize> for Offset {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs;
    }
}

impl std::ops::Sub<Self> for Offset {
    type Output = usize;

    fn sub(self, rhs: Self) -> Self::Output {
        self.index() - rhs.index()
    }
}

/// # Offset Range

#[derive(
    Debug, PartialEq, Default, Eq, PartialOrd, Ord, Clone, Copy, Hash, Serialize, Deserialize,
)]
pub struct OffsetRange {
    start: Offset,
    end: Offset,
}

impl From<Range<Offset>> for OffsetRange {
    fn from(range: Range<Offset>) -> Self {
        Self {
            start: range.start,
            end: range.end,
        }
    }
}

impl Into<Range<usize>> for OffsetRange {
    fn into(self) -> Range<usize> {
        self.raw()
    }
}

impl OffsetRange {
    pub fn new(start: Offset, end: Offset) -> Self {
        Self { start, end }
    }

    pub(crate) fn raw(self) -> Range<usize> {
        self.start.index()..self.end.index()
    }
}

impl OffsetRange {
    pub fn start(self) -> Offset {
        self.start
    }

    pub fn end(self) -> Offset {
        self.end
    }

    pub fn is_empty(self) -> bool {
        self.start == self.end
    }
}

impl std::ops::Index<OffsetRange> for str {
    type Output = str;

    fn index(&self, range: OffsetRange) -> &Self::Output {
        &self[range.start.index()..range.end.index()]
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OffsetRangeFrom {
    start: Offset,
}

impl From<RangeFrom<Offset>> for OffsetRangeFrom {
    fn from(range: RangeFrom<Offset>) -> Self {
        Self { start: range.start }
    }
}

impl std::ops::Index<OffsetRangeFrom> for str {
    type Output = str;

    fn index(&self, range: OffsetRangeFrom) -> &Self::Output {
        &self[range.start.index()..]
    }
}

#[derive(
    Debug, PartialEq, Default, Eq, PartialOrd, Ord, Clone, Copy, Hash, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct OffsetRangeTo {
    end: Offset,
}

impl From<RangeTo<Offset>> for OffsetRangeTo {
    fn from(range: RangeTo<Offset>) -> Self {
        Self { end: range.end }
    }
}

impl OffsetRangeTo {
    pub fn end(self) -> Offset {
        self.end
    }

    pub fn raw(self) -> RangeTo<usize> {
        ..self.end.index()
    }
}

impl std::ops::Index<OffsetRangeTo> for str {
    type Output = str;

    fn index(&self, range: OffsetRangeTo) -> &Self::Output {
        &self[range.raw()]
    }
}
