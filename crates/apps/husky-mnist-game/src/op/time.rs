use super::*;
use shifted_unsigned_int::ShiftedU32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OpTime(ShiftedU32);

impl OpTime {
    pub fn from_index(index: usize) -> Self {
        Self(index.into())
    }

    pub fn index(self) -> usize {
        self.0.into()
    }
}
