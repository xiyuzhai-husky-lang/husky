use std::num::{NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShiftedU8(NonZeroU8);

impl Default for ShiftedU8 {
    fn default() -> Self {
        0usize.into()
    }
}

impl From<usize> for ShiftedU8 {
    fn from(value: usize) -> Self {
        debug_assert!(value + 1 < u8::MAX as usize);
        ShiftedU8(unsafe { NonZeroU8::new_unchecked((value + 1) as u8) })
    }
}

impl Into<usize> for ShiftedU8 {
    fn into(self) -> usize {
        self.0.get() as usize - 1
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ShiftedU16(NonZeroU16);

impl Default for ShiftedU16 {
    fn default() -> Self {
        0usize.into()
    }
}

impl From<usize> for ShiftedU16 {
    fn from(value: usize) -> Self {
        debug_assert!(value + 1 < u16::MAX as usize);
        ShiftedU16(unsafe { NonZeroU16::new_unchecked((value + 1) as u16) })
    }
}

impl Into<usize> for ShiftedU16 {
    fn into(self) -> usize {
        self.0.get() as usize - 1
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShiftedU32(NonZeroU32);

impl Default for ShiftedU32 {
    fn default() -> Self {
        0usize.into()
    }
}

impl From<usize> for ShiftedU32 {
    fn from(value: usize) -> Self {
        debug_assert!(value + 1 < u32::MAX as usize);
        ShiftedU32(unsafe { NonZeroU32::new_unchecked((value + 1) as u32) })
    }
}

impl Into<usize> for ShiftedU32 {
    fn into(self) -> usize {
        self.0.get() as usize - 1
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShiftedU64(NonZeroU64);

impl Default for ShiftedU64 {
    fn default() -> Self {
        0usize.into()
    }
}

impl From<usize> for ShiftedU64 {
    fn from(value: usize) -> Self {
        debug_assert!(value + 1 < u64::MAX as usize);
        ShiftedU64(unsafe { NonZeroU64::new_unchecked((value + 1) as u64) })
    }
}

impl Into<usize> for ShiftedU64 {
    fn into(self) -> usize {
        self.0.get() as usize - 1
    }
}
