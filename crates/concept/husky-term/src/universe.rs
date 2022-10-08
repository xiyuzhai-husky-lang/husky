#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Universe(u8);

const UNIVERSE_MAX: u8 = 100;

impl Universe {
    pub(crate) fn zero() -> Self {
        Universe(0)
    }

    pub(crate) fn next(&self) -> Self {
        assert!(self.0 < UNIVERSE_MAX);
        Universe(self.0 + 1)
    }
}
