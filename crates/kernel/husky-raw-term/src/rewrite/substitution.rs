use super::*;

pub struct RawTermSubstitution {
    src: RawTermVariable,
    dst: RawTerm,
}

impl RawTermSubstitution {
    pub fn src(&self) -> RawTermVariable {
        self.src
    }

    pub fn dst(&self) -> RawTerm {
        self.dst
    }
}
