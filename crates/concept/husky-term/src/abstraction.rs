use crate::*;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct TermAbstraction {
    x: i32,
    m: TermPtr,
}

impl TermAbstraction {
    pub fn ty(&self) -> Ty {
        todo!()
    }

    pub fn universe(&self) -> UniverseLevel {
        todo!()
    }
}
