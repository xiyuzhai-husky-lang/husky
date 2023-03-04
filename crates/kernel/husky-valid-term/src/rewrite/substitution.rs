use super::*;
use husky_word::Identifier;

pub struct ValidTermSubstitution {
    src: ValidTermSymbol,
    dst: ValidTerm,
}

impl ValidTermSubstitution {
    pub fn src(&self) -> ValidTermSymbol {
        self.src
    }

    pub fn dst(&self) -> ValidTerm {
        self.dst
    }
}
