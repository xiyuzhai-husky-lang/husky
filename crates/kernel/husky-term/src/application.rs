use crate::*;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct TermApplication {
    m: TermPtr,
    n: TermPtr,
}

impl TermApplication {
    pub fn ty(&self) -> Ty {
        todo!()
    }

    // pub fn universe(&self) -> TermUniverseLevel {
    //     todo!()
    // }
}
