use crate::*;

#[salsa::interned(jar = TermJar)]
pub struct TermAbstraction {
    x: i32,
    m: Term,
}

impl TermAbstraction {
    pub fn ty(&self) -> Term {
        todo!()
    }

    // pub fn universe(&self) -> TermUniverseLevel {
    //     todo!()
    // }
}

impl std::fmt::Display for TermAbstraction {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
