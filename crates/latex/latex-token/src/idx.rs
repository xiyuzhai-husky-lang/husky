use shifted_unsigned_int::ShiftedU32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TexTokenIdx(ShiftedU32);

impl TexTokenIdx {
    pub(crate) fn from_index(index: usize) -> TexTokenIdx {
        Self(index.into())
    }

    pub(crate) fn index(self) -> usize {
        self.0.index()
    }
}
