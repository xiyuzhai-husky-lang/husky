use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PreciseTermUniverse(u8);

impl From<u8> for PreciseTermUniverse {
    fn from(value: u8) -> Self {
        PreciseTermUniverse::new(value)
    }
}

const UNIVERSE_MAX: u8 = 100;

impl PreciseTermUniverse {
    pub fn new(i: u8) -> Self {
        assert!(i < UNIVERSE_MAX);
        PreciseTermUniverse(i)
    }

    pub(crate) fn zero() -> Self {
        PreciseTermUniverse(0)
    }

    pub fn raw(self) -> u8 {
        self.0
    }

    pub(crate) fn next(self) -> PreciseTermResult<Self> {
        if !(self.0 < UNIVERSE_MAX) {
            return Err(PreciseTermError::UniverseOverflow);
        }
        Ok(PreciseTermUniverse(self.0 + 1))
    }

    // pub(crate) fn prev(self) -> Option<Self> {
    //     if self.0 == 0 {
    //         return None;
    //     }
    //     Some(Universe(self.0 - 1))
    // }

    pub(crate) fn max(self, other: PreciseTermUniverse) -> PreciseTermUniverse {
        PreciseTermUniverse(self.0.max(other.0))
    }

    pub(crate) fn positive(self) -> bool {
        self.0 > 0
    }
}

impl std::fmt::Display for PreciseTermUniverse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
