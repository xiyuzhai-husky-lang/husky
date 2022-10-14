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

    // pub fn universe(&self) -> TermUniverseLevel {
    //     todo!()
    // }
}

impl std::fmt::Display for TermAbstraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
