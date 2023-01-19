use super::*;
use husky_word::Identifier;

pub struct TermSubstitution {
    src: TermSymbol,
    dst: Term,
}

impl TermSubstitution {
    pub fn src(&self) -> TermSymbol {
        self.src
    }

    pub fn dst(&self) -> Term {
        self.dst
    }
}
