use husky_word::Identifier;

use crate::*;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct TermVariable {
    opt_namespace: Optioned<TermNamespace>,
    name: Identifier,
}

impl TermVariable {
    pub fn ty(&self) -> Ty {
        todo!()
    }

    pub fn universe(&self) -> TermUniverseLevel {
        todo!()
    }
}
