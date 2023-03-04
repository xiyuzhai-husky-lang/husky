use super::*;
use husky_word::Identifier;

pub struct RawTermSubstitution {
    src: RawTermSymbol,
    dst: RawTerm,
}

impl RawTermSubstitution {
    pub fn src(&self) -> RawTermSymbol {
        self.src
    }

    pub fn dst(&self) -> RawTerm {
        self.dst
    }
}
