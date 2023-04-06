use super::*;

pub struct RawTermSubstitution {
    src: RawTermHole,
    dst: RawTerm,
}

impl RawTermSubstitution {
    pub fn src(&self) -> RawTermHole {
        self.src
    }

    pub fn dst(&self) -> RawTerm {
        self.dst
    }
}
