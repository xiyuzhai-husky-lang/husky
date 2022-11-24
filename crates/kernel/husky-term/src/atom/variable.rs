use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TermVariableVariant {}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct TermVariable(Term);

impl TermVariable {
    pub fn ty(&self) -> Term {
        todo!()
    }

    // pub fn universe(&self) -> TermUniverseLevel {
    //     todo!()
    // }
}

impl std::fmt::Display for TermVariableVariant {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::fmt::Display for TermVariable {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
