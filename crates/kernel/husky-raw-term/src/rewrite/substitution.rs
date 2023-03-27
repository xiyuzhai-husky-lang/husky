use super::*;

pub struct RawTermSubstitution {
    src: RawTermAbstractSymbol,
    dst: RawTerm,
}

impl RawTermSubstitution {
    pub fn src(&self) -> RawTermAbstractSymbol {
        self.src
    }

    pub fn dst(&self) -> RawTerm {
        self.dst
    }
}
