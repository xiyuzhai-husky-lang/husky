use super::*;
use husky_word::Identifier;

pub struct TermSubstitution {
    src: Term,
    dst: Term,
}

impl TermSubstitution {
    pub fn src(&self) -> Term {
        self.src
    }

    pub fn dst(&self) -> Term {
        self.dst
    }
}
