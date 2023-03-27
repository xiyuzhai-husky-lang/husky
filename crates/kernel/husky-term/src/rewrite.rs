use super::*;
use husky_word::Ident;

pub struct TermSubstitution {
    src: TermConcreteSymbol,
    dst: Term,
}

impl TermSubstitution {
    pub fn src(&self) -> TermConcreteSymbol {
        self.src
    }

    pub fn dst(&self) -> Term {
        self.dst
    }
}
