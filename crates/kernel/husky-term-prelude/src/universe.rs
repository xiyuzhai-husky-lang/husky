use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UniverseTerm(u8);

impl From<u8> for UniverseTerm {
    fn from(value: u8) -> Self {
        UniverseTerm::new(value)
    }
}

const UNIVERSE_MAX: u8 = 100;

impl UniverseTerm {
    pub const PROP_UNIVERSE: Self = UniverseTerm(0);
    pub const TYPE_UNIVERSE: Self = UniverseTerm(1);

    pub const fn new(i: u8) -> Self {
        assert!(i < UNIVERSE_MAX);
        UniverseTerm(i)
    }

    #[inline(always)]
    pub fn from_(_db: &::salsa::Db, _term: UniverseTerm) -> Self {
        UniverseTerm::new(_term.raw())
    }

    pub fn raw(self) -> u8 {
        self.0
    }

    pub(crate) fn next(self) -> TermPreludeResult<Self> {
        if !(self.0 < UNIVERSE_MAX) {
            return Err(TermPreludeError::UniverseOverflow);
        }
        Ok(UniverseTerm(self.0 + 1))
    }
}

impl std::fmt::Display for UniverseTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}
