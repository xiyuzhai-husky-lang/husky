use crate::error::{TermError, TermResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Universe(u8);

const UNIVERSE_MAX: u8 = 100;

impl Universe {
    pub(crate) fn zero() -> Self {
        Universe(0)
    }

    pub(crate) fn next(self) -> TermResult<Self> {
        if !(self.0 < UNIVERSE_MAX) {
            return Err(TermError::UniverseOverflow);
        }
        Ok(Universe(self.0 + 1))
    }

    // pub(crate) fn prev(self) -> Option<Self> {
    //     if self.0 == 0 {
    //         return None;
    //     }
    //     Some(Universe(self.0 - 1))
    // }

    pub(crate) fn max(self, other: Universe) -> Universe {
        Universe(self.0.max(other.0))
    }

    pub(crate) fn positive(self) -> bool {
        self.0 > 0
    }
}
