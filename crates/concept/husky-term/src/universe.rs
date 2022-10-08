use crate::error::{TermError, TermResult};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct TermUniverse {
    pub kind: UniverseKind,
    pub level: UniverseLevel,
}

impl TermUniverse {
    pub fn kind(&self) -> UniverseKind {
        self.kind
    }

    pub(crate) fn ty_universe(&self) -> TermUniverse {
        TermUniverse {
            kind: UniverseKind::Type,
            level: self.level.next().expect("todo"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum UniverseKind {
    Type,
    Sort,
    Term,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UniverseLevel(u8);

const UNIVERSE_MAX: u8 = 100;

impl UniverseLevel {
    pub(crate) fn zero() -> Self {
        UniverseLevel(0)
    }

    pub(crate) fn next(self) -> TermResult<Self> {
        if !(self.0 < UNIVERSE_MAX) {
            return Err(TermError::UniverseOverflow);
        }
        Ok(UniverseLevel(self.0 + 1))
    }

    // pub(crate) fn prev(self) -> Option<Self> {
    //     if self.0 == 0 {
    //         return None;
    //     }
    //     Some(Universe(self.0 - 1))
    // }

    pub(crate) fn max(self, other: UniverseLevel) -> UniverseLevel {
        UniverseLevel(self.0.max(other.0))
    }

    pub(crate) fn positive(self) -> bool {
        self.0 > 0
    }
}
