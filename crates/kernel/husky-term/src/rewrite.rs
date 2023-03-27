use super::*;
use husky_word::Ident;

pub struct TermSubstitution {
    src: TermOriginalVariable,
    dst: Term,
}

impl TermSubstitution {
    pub fn src(&self) -> TermOriginalVariable {
        self.src
    }

    pub fn dst(&self) -> Term {
        self.dst
    }
}
