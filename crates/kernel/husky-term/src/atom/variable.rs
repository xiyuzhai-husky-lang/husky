use husky_word::Identifier;

use crate::*;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum TermVariableVariant {}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct TermVariable(TermItd);

impl TermVariable {
    pub fn ty(&self) -> Ty {
        todo!()
    }

    // pub fn universe(&self) -> TermUniverseLevel {
    //     todo!()
    // }
}

impl std::fmt::Display for TermVariableVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::fmt::Display for TermVariable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
