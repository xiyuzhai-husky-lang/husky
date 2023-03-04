use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RawTermUniverse(u8);

impl From<u8> for RawTermUniverse {
    fn from(value: u8) -> Self {
        RawTermUniverse::new(value)
    }
}

const UNIVERSE_MAX: u8 = 100;

impl RawTermUniverse {
    pub fn new(i: u8) -> Self {
        assert!(i < UNIVERSE_MAX);
        RawTermUniverse(i)
    }

    pub(crate) fn zero() -> Self {
        RawTermUniverse(0)
    }

    pub fn raw(self) -> u8 {
        self.0
    }

    pub(crate) fn next(self) -> RawTermResult<Self> {
        if !(self.0 < UNIVERSE_MAX) {
            return Err(RawTermError::UniverseOverflow);
        }
        Ok(RawTermUniverse(self.0 + 1))
    }

    // pub(crate) fn prev(self) -> Option<Self> {
    //     if self.0 == 0 {
    //         return None;
    //     }
    //     Some(Universe(self.0 - 1))
    // }

    pub(crate) fn max(self, other: RawTermUniverse) -> RawTermUniverse {
        RawTermUniverse(self.0.max(other.0))
    }

    pub(crate) fn positive(self) -> bool {
        self.0 > 0
    }
}

impl std::fmt::Display for RawTermUniverse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
