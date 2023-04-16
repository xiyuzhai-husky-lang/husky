use super::*;
use husky_word::Ident;

pub struct TermSubstitution {
    src: EtherealTermSymbol,
    dst: EtherealTerm,
}

impl TermSubstitution {
    pub fn src(&self) -> EtherealTermSymbol {
        self.src
    }

    pub fn dst(&self) -> EtherealTerm {
        self.dst
    }
}
