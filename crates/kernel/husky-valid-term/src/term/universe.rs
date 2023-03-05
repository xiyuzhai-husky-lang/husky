use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ValidTermUniverse(u8);

impl From<u8> for ValidTermUniverse {
    fn from(value: u8) -> Self {
        ValidTermUniverse::new(value)
    }
}

const UNIVERSE_MAX: u8 = 100;

impl ValidTermUniverse {
    pub fn new(i: u8) -> Self {
        assert!(i < UNIVERSE_MAX);
        ValidTermUniverse(i)
    }

    pub fn from_precise(db: &dyn PreciseTermDb, precise_term: PreciseTermUniverse) -> Self {
        ValidTermUniverse(precise_term.raw())
    }

    pub(crate) fn zero() -> Self {
        ValidTermUniverse(0)
    }

    pub fn raw(self) -> u8 {
        self.0
    }

    pub(crate) fn next(self) -> ValidTermResult<Self> {
        if !(self.0 < UNIVERSE_MAX) {
            return Err(ValidTermError::UniverseOverflow);
        }
        Ok(ValidTermUniverse(self.0 + 1))
    }

    // pub(crate) fn prev(self) -> Option<Self> {
    //     if self.0 == 0 {
    //         return None;
    //     }
    //     Some(Universe(self.0 - 1))
    // }

    pub(crate) fn max(self, other: ValidTermUniverse) -> ValidTermUniverse {
        ValidTermUniverse(self.0.max(other.0))
    }

    pub(crate) fn positive(self) -> bool {
        self.0 > 0
    }
}

impl std::fmt::Display for ValidTermUniverse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}
