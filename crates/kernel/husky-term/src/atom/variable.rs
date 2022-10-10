use husky_word::Identifier;

use crate::*;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum TermVariableVariant {}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct TermVariable(TermPtr);

impl TermVariable {
    pub fn ty(&self) -> Ty {
        todo!()
    }

    // pub fn universe(&self) -> TermUniverseLevel {
    //     todo!()
    // }
}
