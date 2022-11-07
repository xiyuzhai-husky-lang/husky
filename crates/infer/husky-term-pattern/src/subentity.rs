use crate::*;
use husky_word::Identifier;

#[derive(Debug, PartialEq, Eq)]
pub struct TermSubentityPattern {
    parent: TermPatternItd,
    ident: Identifier,
}

impl TermSubentityPattern {
    pub fn parent(&self) -> TermPatternItd {
        self.parent
    }

    pub fn ident(&self) -> Identifier {
        self.ident
    }
}
