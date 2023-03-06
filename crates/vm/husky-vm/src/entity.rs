#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityUid(u64);

impl EntityUid {
    pub unsafe fn from_raw_unchecked(raw: u64) -> EntityUid {
        Self(raw)
    }

    pub fn raw(&self) -> u64 {
        self.0
    }
}
