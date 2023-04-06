use super::*;

pub struct RawTermSubstitution {
    src: RawTermPlaceholder,
    dst: RawTerm,
}

impl RawTermSubstitution {
    pub fn src(&self) -> RawTermPlaceholder {
        self.src
    }

    pub fn dst(&self) -> RawTerm {
        self.dst
    }
}
