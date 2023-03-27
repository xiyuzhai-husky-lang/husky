use super::*;

pub struct RawTermSubstitution {
    src: RawTermDerivedVariable,
    dst: RawTerm,
}

impl RawTermSubstitution {
    pub fn src(&self) -> RawTermDerivedVariable {
        self.src
    }

    pub fn dst(&self) -> RawTerm {
        self.dst
    }
}
