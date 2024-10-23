use shifted_unsigned_int::ShiftedU32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LxTokenIdx(ShiftedU32);

impl LxTokenIdx {
    pub(crate) fn from_index(index: usize) -> LxTokenIdx {
        Self(index.into())
    }

    pub(crate) fn index(self) -> usize {
        self.0.index()
    }
}
