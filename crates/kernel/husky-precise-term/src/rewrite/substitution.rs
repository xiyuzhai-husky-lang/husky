use super::*;
use husky_word::Identifier;

pub struct PreciseTermSubstitution {
    src: PreciseTermSymbol,
    dst: PreciseTerm,
}

impl PreciseTermSubstitution {
    pub fn src(&self) -> PreciseTermSymbol {
        self.src
    }

    pub fn dst(&self) -> PreciseTerm {
        self.dst
    }
}
