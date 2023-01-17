use super::*;
use husky_word::Identifier;

pub enum TermSubstitution {
    Variable { src: Identifier, dst: Term },
    Lifetime { src: Identifier, dst: Identifier },
    Binding { src: Identifier, dst: Identifier },
}
