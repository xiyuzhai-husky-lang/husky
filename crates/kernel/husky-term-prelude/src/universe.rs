use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Universe(u8);

impl From<u8> for Universe {
    fn from(value: u8) -> Self {
        Universe::new(value)
    }
}

const UNIVERSE_MAX: u8 = 100;

impl Universe {
    pub const PROP_UNIVERSE: Self = Universe(0);
    pub const TYPE_UNIVERSE: Self = Universe(1);

    pub const fn new(i: u8) -> Self {
        assert!(i < UNIVERSE_MAX);
        Universe(i)
    }

    #[inline(always)]
    pub fn from_(_db: &::salsa::Db, _term: Universe) -> Self {
        Universe::new(_term.raw())
    }

    pub fn raw(self) -> u8 {
        self.0
    }

    pub(crate) fn next(self) -> TermPreludeResult<Self> {
        if !(self.0 < UNIVERSE_MAX) {
            return Err(TermPreludeError::UniverseOverflow);
        }
        Ok(Universe(self.0 + 1))
    }
}

impl std::fmt::Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}
