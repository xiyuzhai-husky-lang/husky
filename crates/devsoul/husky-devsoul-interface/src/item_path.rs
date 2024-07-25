use shifted_unsigned_int::ShiftedU32;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct ItemPathIdInterface(ShiftedU32);

impl ItemPathIdInterface {
    pub fn new(index: u32) -> Self {
        Self(index.into())
    }
}
