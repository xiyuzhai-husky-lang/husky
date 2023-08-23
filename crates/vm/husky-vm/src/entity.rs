#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityUid(u32);

impl EntityUid {
    pub unsafe fn from_raw(raw: u32) -> EntityUid {
        Self(raw)
    }

    pub fn raw(&self) -> u32 {
        self.0
    }
}
