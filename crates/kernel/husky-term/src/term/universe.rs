use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TermUniverse(u8);

impl From<TermUniverse> for Term {
    fn from(val: TermUniverse) -> Self {
        Term::Universe(val)
    }
}

const UNIVERSE_MAX: u8 = 100;

impl TermUniverse {
    pub(crate) fn new(i: u8) -> Self {
        assert!(i < UNIVERSE_MAX);
        TermUniverse(i)
    }

    pub(crate) fn zero() -> Self {
        TermUniverse(0)
    }

    pub fn raw(self) -> u8 {
        self.0
    }

    pub(crate) fn next(self) -> TermResult<Self> {
        if !(self.0 < UNIVERSE_MAX) {
            return Err(TermError::UniverseOverflow);
        }
        Ok(TermUniverse(self.0 + 1))
    }

    // pub(crate) fn prev(self) -> Option<Self> {
    //     if self.0 == 0 {
    //         return None;
    //     }
    //     Some(Universe(self.0 - 1))
    // }

    pub(crate) fn max(self, other: TermUniverse) -> TermUniverse {
        TermUniverse(self.0.max(other.0))
    }

    pub(crate) fn positive(self) -> bool {
        self.0 > 0
    }
}

impl std::fmt::Display for TermUniverse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
